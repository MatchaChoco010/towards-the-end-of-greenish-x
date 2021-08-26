use proc_macro2::TokenStream;
use quote::quote;

struct Items(Vec<syn::Item>);
impl syn::parse::Parse for Items {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut items = vec![];
        while !input.is_empty() {
            match input.parse::<syn::Item>() {
                Ok(item) => items.push(item),
                Err(err) => return Err(err),
            }
        }
        Ok(Items(items))
    }
}

struct PropertyInfo {
    enum_name: Option<String>,
    struct_name: String,
    property_name: String,
}
struct GatherInfo {
    properties: Vec<PropertyInfo>,
}
fn gather_attr_info(
    items: &mut Vec<syn::Item>,
) -> std::result::Result<GatherInfo, Vec<syn::Error>> {
    let animation_component_attr = syn::parse2::<syn::Path>(quote!(animation_component)).unwrap();
    let animation_property_attr = syn::parse2::<syn::Path>(quote!(animation_property)).unwrap();
    let type_f32 = syn::parse2::<syn::Type>(quote!(f32)).unwrap();

    let mut properties = vec![];
    let mut errors = vec![];

    for item in items.iter_mut() {
        match item {
            syn::Item::Struct(s) if s.attrs.iter().any(|a| a.path == animation_component_attr) => {
                for field in s.fields.iter_mut() {
                    if field.ident.is_none() {
                        errors.push(syn::Error::new_spanned(field.ty.clone(), "In the current implementation, tuple structures cannot be used for animation properties"));
                        break;
                    }
                    if field
                        .attrs
                        .iter()
                        .any(|a| a.path == animation_property_attr)
                    {
                        if field.ty != type_f32 {
                            errors.push(syn::Error::new_spanned(field.ty.clone(), "In the current implementation, only the f32 type can be used for animation properties"));
                            break;
                        }
                        s.attrs = s
                            .attrs
                            .drain(0..)
                            .filter(|a| a.path != animation_component_attr)
                            .collect::<Vec<_>>();
                        field.attrs = field
                            .attrs
                            .drain(0..)
                            .filter(|a| a.path != animation_property_attr)
                            .collect::<Vec<_>>();
                        let struct_name = s.ident.to_string();
                        let property_name = field.ident.as_ref().unwrap().to_string();
                        properties.push(PropertyInfo {
                            enum_name: None,
                            struct_name,
                            property_name,
                        })
                    }
                }
            }
            syn::Item::Enum(e) => {
                for v in e.variants.iter_mut() {
                    if !v.attrs.iter().any(|a| a.path == animation_component_attr) {
                        continue;
                    }
                    for field in v.fields.iter_mut() {
                        if field.ident.is_none() {
                            errors.push(syn::Error::new_spanned(field.ty.clone(), "In the current implementation, tuple structure variants cannot be used for animation properties"));
                            break;
                        }
                        if field
                            .attrs
                            .iter()
                            .any(|a| a.path == animation_property_attr)
                        {
                            if field.ty != type_f32 {
                                errors.push(syn::Error::new_spanned(field.ty.clone(), "In the current implementation, only the f32 type can be used for animation properties"));
                                break;
                            }
                            v.attrs = v
                                .attrs
                                .drain(0..)
                                .filter(|a| a.path != animation_component_attr)
                                .collect::<Vec<_>>();
                            field.attrs = field
                                .attrs
                                .drain(0..)
                                .filter(|a| a.path != animation_property_attr)
                                .collect::<Vec<_>>();
                            let enum_name = Some(e.ident.to_string());
                            let struct_name = v.ident.to_string();
                            let property_name = field.ident.as_ref().unwrap().to_string();
                            properties.push(PropertyInfo {
                                enum_name,
                                struct_name,
                                property_name,
                            })
                        }
                    }
                }
            }
            _ => (),
        }
    }

    if errors.is_empty() {
        Ok(GatherInfo { properties })
    } else {
        Err(errors)
    }
}

fn animation_property_type_token(gather_info: &GatherInfo) -> TokenStream {
    let mut property_type_enum = syn::parse2::<syn::ItemEnum>(quote! {
            #[derive(::serde::Deserialize, Debug, Clone, Copy)]
            enum AnimationPropertyType {}
    })
    .unwrap();
    let mut punctuated_variant = syn::punctuated::Punctuated::new();
    for prop in &gather_info.properties {
        let prop_name = if let Some(enum_name) = prop.enum_name.as_ref() {
            syn::Ident::new(
                &(enum_name.to_owned() + "_" + &prop.struct_name + "_" + &prop.property_name),
                proc_macro2::Span::call_site(),
            )
        } else {
            syn::Ident::new(
                &(prop.struct_name.to_owned() + "_" + &prop.property_name),
                proc_macro2::Span::call_site(),
            )
        };
        let variant: syn::Variant = syn::parse_quote! {
            #[allow(non_camel_case_types)]
            #prop_name
        };
        punctuated_variant.push(variant);
    }
    property_type_enum.variants = punctuated_variant;
    quote!(#property_type_enum)
}

fn animation_property_components(gather_info: &GatherInfo) -> TokenStream {
    let mut token_stream = quote!();

    for prop in &gather_info.properties {
        let enum_name = if let Some(enum_name) = &prop.enum_name {
            Some(syn::Ident::new(&enum_name, proc_macro2::Span::call_site()))
        } else {
            None
        };
        let struct_name = syn::Ident::new(&prop.struct_name, proc_macro2::Span::call_site());
        let property_name = syn::Ident::new(&prop.property_name, proc_macro2::Span::call_site());

        if let Some(enum_name_str) = &prop.enum_name {
            let component_name = syn::Ident::new(
                &("AnimationPropertyComponent_".to_string()
                    + enum_name_str
                    + "_"
                    + &prop.struct_name
                    + "_"
                    + &prop.property_name),
                proc_macro2::Span::call_site(),
            );

            token_stream.extend(quote! {
                #[allow(non_camel_case_types)]
                struct #component_name {
                    uuid: ::uuid::Uuid,
                }
                impl #component_name {
                    fn update(
                        &self,
                        anim_component: &EntityAnimationComponent,
                        component: &mut #enum_name,
                        animation_store: &AnimationStore,
                    ) {
                        match component {
                            #enum_name::#struct_name { #property_name, .. } => {
                                let (_, anim) = animation_store.float_32_animations.get(&self.uuid).unwrap();
                                let range = anim.get_range(anim_component.current_frame);
                                let value = range.get_value(
                                    anim_component.current_time - anim_component.start_time,
                                    anim_component.fps,
                                );

                                *#property_name = value;
                            }
                            _ => (),
                        }
                    }
                }
            });
        } else {
            let component_name = syn::Ident::new(
                &("AnimationPropertyComponent_".to_string()
                    + &prop.struct_name
                    + "_"
                    + &prop.property_name),
                proc_macro2::Span::call_site(),
            );

            token_stream.extend(quote! {
                #[allow(non_camel_case_types)]
                struct #component_name {
                    uuid: ::uuid::Uuid,
                }
                impl #component_name {
                    fn update(
                        &self,
                        anim_component: &EntityAnimationComponent,
                        component: &mut #struct_name,
                        animation_store: &AnimationStore,
                    ) {
                        let (_, anim) = animation_store.float_32_animations.get(&self.uuid).unwrap();
                        let range = anim.get_range(anim_component.current_frame);
                        let value = range.get_value(
                            anim_component.current_time - anim_component.start_time,
                            anim_component.fps,
                        );

                        component.#property_name = value;
                    }
                }
            });
        }
    }

    token_stream
}

fn add_animation_property_component(gather_info: &GatherInfo) -> TokenStream {
    let mut token_stream = quote!();

    for prop in &gather_info.properties {
        let component_type_variant = if let Some(enum_name) = &prop.enum_name {
            syn::Ident::new(
                &(enum_name.to_owned() + "_" + &prop.struct_name + "_" + &prop.property_name),
                proc_macro2::Span::call_site(),
            )
        } else {
            syn::Ident::new(
                &(prop.struct_name.to_owned() + "_" + &prop.property_name),
                proc_macro2::Span::call_site(),
            )
        };
        let component_name = if let Some(enum_name) = &prop.enum_name {
            syn::Ident::new(
                &("AnimationPropertyComponent_".to_string()
                    + enum_name
                    + "_"
                    + &prop.struct_name
                    + "_"
                    + &prop.property_name),
                proc_macro2::Span::call_site(),
            )
        } else {
            syn::Ident::new(
                &("AnimationPropertyComponent_".to_string()
                    + &prop.struct_name
                    + "_"
                    + &prop.property_name),
                proc_macro2::Span::call_site(),
            )
        };

        token_stream.extend(quote! {
            AnimationPropertyType::#component_type_variant => {
                entry.add_component(#component_name { uuid })
            }
        });
    }

    quote!(|(uuid, ty)| match ty {
        #token_stream
    })
}

fn remove_animation_property_component(gather_info: &GatherInfo) -> TokenStream {
    let mut token_stream = quote!();

    for prop in &gather_info.properties {
        let component_type_variant = if let Some(enum_name) = &prop.enum_name {
            syn::Ident::new(
                &(enum_name.to_owned() + "_" + &prop.struct_name + "_" + &prop.property_name),
                proc_macro2::Span::call_site(),
            )
        } else {
            syn::Ident::new(
                &(prop.struct_name.to_owned() + "_" + &prop.property_name),
                proc_macro2::Span::call_site(),
            )
        };
        let component_name = if let Some(enum_name) = &prop.enum_name {
            syn::Ident::new(
                &("AnimationPropertyComponent_".to_string()
                    + enum_name
                    + "_"
                    + &prop.struct_name
                    + "_"
                    + &prop.property_name),
                proc_macro2::Span::call_site(),
            )
        } else {
            syn::Ident::new(
                &("AnimationPropertyComponent_".to_string()
                    + &prop.struct_name
                    + "_"
                    + &prop.property_name),
                proc_macro2::Span::call_site(),
            )
        };

        token_stream.extend(quote! {
            AnimationPropertyType::#component_type_variant => {
                commands.remove_component::<#component_name>(*entity)
            }
        });
    }

    quote!(match ty {
        #token_stream
    })
}

fn animation_property_systems(gather_info: &GatherInfo) -> TokenStream {
    let mut token_stream = quote!();

    for prop in &gather_info.properties {
        let system_function_name = if let Some(enum_name) = &prop.enum_name {
            syn::Ident::new(
                &("animation_property_component_".to_string()
                    + enum_name
                    + "_"
                    + &prop.struct_name
                    + "_"
                    + &prop.property_name),
                proc_macro2::Span::call_site(),
            )
        } else {
            syn::Ident::new(
                &("animation_property_component_".to_string()
                    + &prop.struct_name
                    + "_"
                    + &prop.property_name),
                proc_macro2::Span::call_site(),
            )
        };
        let animation_property_component_name = if let Some(enum_name) = &prop.enum_name {
            syn::Ident::new(
                &("AnimationPropertyComponent_".to_string()
                    + enum_name
                    + "_"
                    + &prop.struct_name
                    + "_"
                    + &prop.property_name),
                proc_macro2::Span::call_site(),
            )
        } else {
            syn::Ident::new(
                &("AnimationPropertyComponent_".to_string()
                    + &prop.struct_name
                    + "_"
                    + &prop.property_name),
                proc_macro2::Span::call_site(),
            )
        };
        let ident = if let Some(enum_name) = &prop.enum_name {
            syn::Ident::new(enum_name, proc_macro2::Span::call_site())
        } else {
            syn::Ident::new(&prop.struct_name, proc_macro2::Span::call_site())
        };

        token_stream.extend(quote! {
            #[allow(non_snake_case)]
            #[::legion::system(for_each)]
            fn #system_function_name(
                anim_component: &EntityAnimationComponent,
                animation_property_component: &#animation_property_component_name,
                component: &mut #ident,
                #[resource] animation_store: &AnimationStore,
            ) {
                animation_property_component.update(anim_component, component, animation_store);
            }
        });
    }

    token_stream
}

fn add_animation_property_systems_to_schedule(gather_info: &GatherInfo) -> TokenStream {
    let mut token_stream = quote!();

    for prop in &gather_info.properties {
        let system_function_name = if let Some(enum_name) = &prop.enum_name {
            syn::Ident::new(
                &("animation_property_component_".to_string()
                    + enum_name
                    + "_"
                    + &prop.struct_name
                    + "_"
                    + &prop.property_name
                    + "_system"),
                proc_macro2::Span::call_site(),
            )
        } else {
            syn::Ident::new(
                &("animation_property_component_".to_string()
                    + &prop.struct_name
                    + "_"
                    + &prop.property_name
                    + "_system"),
                proc_macro2::Span::call_site(),
            )
        };

        token_stream.extend(quote! {
            schedule.add_system(#system_function_name());
        });
    }

    token_stream
}

pub fn anim_components(input: TokenStream) -> TokenStream {
    match syn::parse2::<Items>(input) {
        Err(err) => err.to_compile_error(),
        Ok(Items(mut items)) => match gather_attr_info(&mut items) {
            Err(errors) => errors.into_iter().fold(quote!(), |mut ts, err| {
                ts.extend(err.to_compile_error());
                ts
            }),
            Ok(gather_info) => {
                let animation_property_type = animation_property_type_token(&gather_info);
                let animation_property_components = animation_property_components(&gather_info);
                let add_animation_property_component =
                    add_animation_property_component(&gather_info);
                let remove_animation_property_component =
                    remove_animation_property_component(&gather_info);
                let animation_property_systems = animation_property_systems(&gather_info);
                let add_animation_property_systems_to_schedule =
                    add_animation_property_systems_to_schedule(&gather_info);

                quote! {
                    #[derive(Debug, ::thiserror::Error)]
                    pub enum AnimationError {
                        #[error("the entity `{0:?}` is not exists in the world")]
                        NotExistsSuchEntity(::legion::Entity),
                        #[error("the animation `{0}` is not registered yet")]
                        NotRegisteredSuchAnimation(String),
                    }

                    #[derive(::serde::Deserialize, Clone, Copy, Debug)]
                    struct Float32Keyframe {
                        frame: u32,
                        value: f32,
                    }

                    #[derive(Clone, Copy)]
                    struct Float32KeyRange {
                        start: Float32Keyframe,
                        end: Float32Keyframe,
                    }
                    impl Float32KeyRange {
                        fn get_value(&self, duration: ::std::time::Duration, fps: f32) -> f32 {
                            if self.end.frame == u32::MAX {
                                return self.start.value;
                            }

                            let range_len = self.end.frame as f32 / fps - self.start.frame as f32 / fps;
                            let range_duration = duration.as_secs_f32() - self.start.frame as f32 / fps;
                            let t = range_duration / range_len;
                            self.start.value + t * (self.end.value - self.start.value)
                        }
                    }

                    #[derive(::serde::Deserialize, Debug)]
                    struct Float32AnimationData {
                        keys: Vec<Float32Keyframe>,
                    }
                    impl Float32AnimationData {
                        fn get_range(&self, frame: u32) -> Float32KeyRange {
                            let first_frame = Float32Keyframe {
                                frame: 0,
                                value: self.keys[0].value,
                            };
                            let last_frame = Float32Keyframe {
                                frame: u32::MAX,
                                value: self.keys.last().unwrap().value,
                            };
                            [first_frame]
                                .iter()
                                .chain(self.keys.iter())
                                .zip(self.keys.iter().chain([last_frame].iter()))
                                .map(|(&s, &e)| Float32KeyRange { start: s, end: e })
                                .find(|range| range.start.frame <= frame && frame < range.end.frame)
                                .unwrap()
                        }
                    }

                    #animation_property_type

                    #[derive(::serde::Deserialize, Debug)]
                    struct EntityAnimationSerializeData {
                        len: u32,
                        fps: f32,
                        data: Vec<(AnimationPropertyType, Float32AnimationData)>,
                    }

                    struct EntityAnimationData {
                        len: u32,
                        fps: f32,
                        data: Vec<::uuid::Uuid>,
                    }
                    pub struct AnimationStore {
                        float_32_animations: ::std::collections::HashMap<::uuid::Uuid, (AnimationPropertyType, Float32AnimationData)>,
                        entity_animations: ::std::collections::HashMap<String, EntityAnimationData>,
                    }
                    impl AnimationStore {
                        pub fn new() -> Self {
                            Self {
                                float_32_animations: ::std::collections::HashMap::new(),
                                entity_animations: ::std::collections::HashMap::new(),
                            }
                        }

                        pub fn load_animation_yaml(&mut self, name: impl ToString, reader: impl ::std::io::Read) -> ::anyhow::Result<()> {
                            let reader = ::std::io::BufReader::new(reader);
                            let anim_data: EntityAnimationSerializeData = serde_yaml::from_reader(reader)?;

                            let mut data = vec![];
                            for d in anim_data.data {
                                let uuid = ::uuid::Uuid::new_v4();
                                self.float_32_animations.insert(uuid, d);
                                data.push(uuid);
                            }

                            self.entity_animations.insert(
                                name.to_string(),
                                EntityAnimationData {
                                    len: anim_data.len,
                                    fps: anim_data.fps,
                                    data,
                                },
                            );

                            Ok(())
                        }

                        pub fn insert_animation_components(
                            &self,
                            entity: ::legion::Entity,
                            name: impl ToString,
                            world: &mut ::legion::World,
                            start_time: ::std::time::Instant,
                        ) -> ::anyhow::Result<AnimationFinishChecker> {
                            if let Some(mut entry) = world.entry(entity) {
                                if let Some(EntityAnimationData { len, fps, data }) =
                                    self.entity_animations.get(&name.to_string())
                                {
                                    data.iter()
                                        .map(|uuid| (uuid, self.float_32_animations.get(uuid).unwrap()))
                                        .map(|(&uuid, (ty, _))| (uuid, ty))
                                        .for_each(
                                            #add_animation_property_component
                                        );
                                    let (checker, component) = EntityAnimationComponent::new(
                                        *len,
                                        *fps,
                                        start_time,
                                        data.iter()
                                            .map(|uuid| self.float_32_animations.get(uuid).unwrap().0)
                                            .collect::<Vec<_>>(),
                                    );
                                    entry.add_component(component);
                                    Ok(checker)
                                } else {
                                    Err(AnimationError::NotRegisteredSuchAnimation(name.to_string()))?
                                }
                            } else {
                                Err(AnimationError::NotExistsSuchEntity(entity))?
                            }
                        }
                    }

                    struct AnimationFinishSignal;
                    pub struct AnimationFinishChecker {
                        is_finished: bool,
                        receiver: ::std::sync::mpsc::Receiver<AnimationFinishSignal>,
                    }
                    impl AnimationFinishChecker {
                        fn new(receiver: ::std::sync::mpsc::Receiver<AnimationFinishSignal>) -> Self {
                            Self {
                                is_finished: false,
                                receiver,
                            }
                        }

                        pub fn is_finished(&mut self) -> bool {
                            if self.is_finished {
                                true
                            } else {
                                if let Ok(_) = self.receiver.try_recv() {
                                    self.is_finished = true;
                                    true
                                } else {
                                    false
                                }
                            }
                        }
                        pub fn is_playing(&mut self) -> bool {
                            !self.is_finished()
                        }
                    }

                    enum EntityAnimationUpdateStatus {
                        Playing,
                        JustFinish,
                        Finished,
                    }
                    struct EntityAnimationComponent {
                        len: u32,
                        fps: f32,
                        current_frame: u32,
                        start_time: ::std::time::Instant,
                        current_time: ::std::time::Instant,
                        animation_types: Vec<AnimationPropertyType>,
                        is_finished: bool,
                        finish_sender: ::std::sync::Arc<::std::sync::Mutex<::std::sync::mpsc::Sender<AnimationFinishSignal>>>,
                    }
                    impl EntityAnimationComponent {
                        fn new(
                            len: u32,
                            fps: f32,
                            start_time: ::std::time::Instant,
                            animation_types: Vec<AnimationPropertyType>,
                        ) -> (AnimationFinishChecker, Self) {
                            let (sender, receiver) = ::std::sync::mpsc::channel();
                            (
                                AnimationFinishChecker::new(receiver),
                                EntityAnimationComponent {
                                    len,
                                    fps,
                                    current_frame: 0,
                                    start_time,
                                    current_time: start_time,
                                    animation_types,
                                    is_finished: false,
                                    finish_sender: ::std::sync::Arc::new(::std::sync::Mutex::new(sender)),
                                },
                            )
                        }

                        fn update(&mut self, delta_time: ::std::time::Duration) -> EntityAnimationUpdateStatus {
                            self.current_time += delta_time;
                            let duration = self.current_time - self.start_time;
                            let frame = (duration.as_secs_f32() * self.fps).floor() as u32;
                            let frame = frame.min(self.len);
                            self.current_frame = frame;

                            if frame >= self.len {
                                if !self.is_finished {
                                    self.is_finished = true;
                                    let _ = self.finish_sender
                                        .lock()
                                        .unwrap()
                                        .send(AnimationFinishSignal);
                                    EntityAnimationUpdateStatus::JustFinish
                                } else {
                                    EntityAnimationUpdateStatus::Finished
                                }
                            } else {
                                EntityAnimationUpdateStatus::Playing
                            }
                        }
                    }

                    #animation_property_components

                    #[::legion::system(for_each)]
                    fn entity_animation(
                        commands: &mut ::legion::systems::CommandBuffer,
                        entity: &::legion::Entity,
                        component: &mut EntityAnimationComponent,
                        #[resource] delta_time: &::std::time::Duration,
                    ) {
                        let status = component.update(*delta_time);
                        match status {
                            EntityAnimationUpdateStatus::Playing | EntityAnimationUpdateStatus::JustFinish => (),
                            EntityAnimationUpdateStatus::Finished => {
                                commands.remove_component::<EntityAnimationComponent>(*entity);
                                for ty in component.animation_types.iter() {
                                    #remove_animation_property_component
                                }
                            }
                        }
                    }

                    #animation_property_systems

                    pub fn add_animation_system(schedule: &mut ::legion::systems::Builder) {
                        schedule.add_system(entity_animation_system());
                        schedule.flush();

                        #add_animation_property_systems_to_schedule
                    }

                    #(#items)*
                }
            }
        },
    }
}
