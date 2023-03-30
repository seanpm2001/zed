use gpui::{
    actions,
    anyhow::Result,
    color::Color,
    elements::{
        Canvas, Container, ContainerStyle, ElementBox, Flex, Label, Margin, MouseEventHandler,
        Padding, ParentElement,
    },
    fonts::TextStyle,
    serde_json, AppContext, Border, Element, Entity, MutableAppContext, Quad, RenderContext, Task,
    View, ViewContext,
};
use serde::{Deserialize, Serialize};
use settings::Settings;
use theme::{ColorScheme, Layer, Style, StyleSet};
use workspace::{
    item::{Item, PersistentItem},
    store::{Record, Store},
    Workspace,
};

actions!(theme, [DeployThemeTestbench]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(ThemeTestbench::deploy);

    workspace::register_persistent_item::<ThemeTestbench>(cx)
}

pub struct ThemeTestbench {
    record_id: Option<u64>,
}

impl ThemeTestbench {
    pub fn deploy(
        workspace: &mut Workspace,
        _: &DeployThemeTestbench,
        cx: &mut ViewContext<Workspace>,
    ) {
        let view = cx.add_view(|_| ThemeTestbench { record_id: None });
        workspace.add_item(Box::new(view), cx);
    }

    fn render_ramps(color_scheme: &ColorScheme) -> Flex {
        fn display_ramp(ramp: &Vec<Color>) -> ElementBox {
            Flex::row()
                .with_children(ramp.iter().cloned().map(|color| {
                    Canvas::new(move |bounds, _, cx| {
                        cx.scene.push_quad(Quad {
                            bounds,
                            background: Some(color),
                            ..Default::default()
                        });
                    })
                    .flex(1.0, false)
                    .boxed()
                }))
                .flex(1.0, false)
                .boxed()
        }

        Flex::column()
            .with_child(display_ramp(&color_scheme.ramps.neutral))
            .with_child(display_ramp(&color_scheme.ramps.red))
            .with_child(display_ramp(&color_scheme.ramps.orange))
            .with_child(display_ramp(&color_scheme.ramps.yellow))
            .with_child(display_ramp(&color_scheme.ramps.green))
            .with_child(display_ramp(&color_scheme.ramps.cyan))
            .with_child(display_ramp(&color_scheme.ramps.blue))
            .with_child(display_ramp(&color_scheme.ramps.violet))
            .with_child(display_ramp(&color_scheme.ramps.magenta))
    }

    fn render_layer(
        layer_index: usize,
        layer: &Layer,
        cx: &mut RenderContext<'_, Self>,
    ) -> Container {
        Flex::column()
            .with_child(
                Self::render_button_set(0, layer_index, "base", &layer.base, cx)
                    .flex(1., false)
                    .boxed(),
            )
            .with_child(
                Self::render_button_set(1, layer_index, "variant", &layer.variant, cx)
                    .flex(1., false)
                    .boxed(),
            )
            .with_child(
                Self::render_button_set(2, layer_index, "on", &layer.on, cx)
                    .flex(1., false)
                    .boxed(),
            )
            .with_child(
                Self::render_button_set(3, layer_index, "accent", &layer.accent, cx)
                    .flex(1., false)
                    .boxed(),
            )
            .with_child(
                Self::render_button_set(4, layer_index, "positive", &layer.positive, cx)
                    .flex(1., false)
                    .boxed(),
            )
            .with_child(
                Self::render_button_set(5, layer_index, "warning", &layer.warning, cx)
                    .flex(1., false)
                    .boxed(),
            )
            .with_child(
                Self::render_button_set(6, layer_index, "negative", &layer.negative, cx)
                    .flex(1., false)
                    .boxed(),
            )
            .contained()
            .with_style(ContainerStyle {
                margin: Margin {
                    top: 10.,
                    bottom: 10.,
                    left: 10.,
                    right: 10.,
                },
                background_color: Some(layer.base.default.background),
                ..Default::default()
            })
    }

    fn render_button_set(
        set_index: usize,
        layer_index: usize,
        set_name: &'static str,
        style_set: &StyleSet,
        cx: &mut RenderContext<'_, Self>,
    ) -> Flex {
        Flex::row()
            .with_child(Self::render_button(
                set_index * 6,
                layer_index,
                set_name,
                &style_set,
                None,
                cx,
            ))
            .with_child(Self::render_button(
                set_index * 6 + 1,
                layer_index,
                "hovered",
                &style_set,
                Some(|style_set| &style_set.hovered),
                cx,
            ))
            .with_child(Self::render_button(
                set_index * 6 + 2,
                layer_index,
                "pressed",
                &style_set,
                Some(|style_set| &style_set.pressed),
                cx,
            ))
            .with_child(Self::render_button(
                set_index * 6 + 3,
                layer_index,
                "active",
                &style_set,
                Some(|style_set| &style_set.active),
                cx,
            ))
            .with_child(Self::render_button(
                set_index * 6 + 4,
                layer_index,
                "disabled",
                &style_set,
                Some(|style_set| &style_set.disabled),
                cx,
            ))
            .with_child(Self::render_button(
                set_index * 6 + 5,
                layer_index,
                "inverted",
                &style_set,
                Some(|style_set| &style_set.inverted),
                cx,
            ))
    }

    fn render_button(
        button_index: usize,
        layer_index: usize,
        text: &'static str,
        style_set: &StyleSet,
        style_override: Option<fn(&StyleSet) -> &Style>,
        cx: &mut RenderContext<'_, Self>,
    ) -> ElementBox {
        enum TestBenchButton {}
        MouseEventHandler::<TestBenchButton>::new(layer_index + button_index, cx, |state, cx| {
            let style = if let Some(style_override) = style_override {
                style_override(&style_set)
            } else if state.clicked().is_some() {
                &style_set.pressed
            } else if state.hovered() {
                &style_set.hovered
            } else {
                &style_set.default
            };

            Self::render_label(text.to_string(), style, cx)
                .contained()
                .with_style(ContainerStyle {
                    margin: Margin {
                        top: 4.,
                        bottom: 4.,
                        left: 4.,
                        right: 4.,
                    },
                    padding: Padding {
                        top: 4.,
                        bottom: 4.,
                        left: 4.,
                        right: 4.,
                    },
                    background_color: Some(style.background),
                    border: Border {
                        width: 1.,
                        color: style.border,
                        overlay: false,
                        top: true,
                        bottom: true,
                        left: true,
                        right: true,
                    },
                    corner_radius: 2.,
                    ..Default::default()
                })
                .boxed()
        })
        .flex(1., true)
        .boxed()
    }

    fn render_label(text: String, style: &Style, cx: &mut RenderContext<'_, Self>) -> Label {
        let settings = cx.global::<Settings>();
        let font_cache = cx.font_cache();
        let family_id = settings.buffer_font_family;
        let font_size = settings.buffer_font_size;
        let font_id = font_cache
            .select_font(family_id, &Default::default())
            .unwrap();

        let text_style = TextStyle {
            color: style.foreground,
            font_family_id: family_id,
            font_family_name: font_cache.family_name(family_id).unwrap(),
            font_id,
            font_size,
            font_properties: Default::default(),
            underline: Default::default(),
        };

        Label::new(text, text_style)
    }
}

impl Entity for ThemeTestbench {
    type Event = ();
}

impl View for ThemeTestbench {
    fn ui_name() -> &'static str {
        "ThemeTestbench"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        let color_scheme = &cx.global::<Settings>().theme.clone().color_scheme;

        Flex::row()
            .with_child(
                Self::render_ramps(color_scheme)
                    .contained()
                    .with_margin_right(10.)
                    .flex(0.1, false)
                    .boxed(),
            )
            .with_child(
                Flex::column()
                    .with_child(
                        Self::render_layer(100, &color_scheme.lowest, cx)
                            .flex(1., true)
                            .boxed(),
                    )
                    .with_child(
                        Self::render_layer(200, &color_scheme.middle, cx)
                            .flex(1., true)
                            .boxed(),
                    )
                    .with_child(
                        Self::render_layer(300, &color_scheme.highest, cx)
                            .flex(1., true)
                            .boxed(),
                    )
                    .flex(1., false)
                    .boxed(),
            )
            .boxed()
    }
}

impl Item for ThemeTestbench {
    fn tab_content(
        &self,
        _: Option<usize>,
        style: &theme::Tab,
        _: &AppContext,
    ) -> gpui::ElementBox {
        Label::new("Theme Testbench", style.label.clone())
            .aligned()
            .contained()
            .boxed()
    }
}

impl PersistentItem for ThemeTestbench {
    type State = ThemeTestbenchState;

    fn save_state(&self, store: Store, cx: &mut ViewContext<Self>) -> Task<Result<u64>> {
        if let Some(record_id) = self.record_id {
            Task::ready(Ok(record_id))
        } else {
            cx.spawn(|this, mut cx| async move {
                let record_id = store.create(&ThemeTestbenchState {}).await?;
                this.update(&mut cx, |this, _| this.record_id = Some(record_id));
                Ok(record_id)
            })
        }
    }

    fn build_from_state(state: Self::State, cx: &mut ViewContext<Self>) -> Self {
        todo!()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ThemeTestbenchState {}

impl Record for ThemeTestbenchState {
    fn namespace() -> &'static str {
        "ThemeTestbenchState"
    }

    fn schema_version() -> u64 {
        0
    }

    fn serialize(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    fn deserialize(_: u64, data: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(serde_json::from_slice(&data)?)
    }
}
