use iced::{
    alignment::Vertical,
    font::{Family, Weight},
    theme,
    widget::{self, container, image},
    BorderRadius, Color, Font, Length, Padding, Sandbox, Settings,
};

fn main() -> iced::Result {
    ExampleApp::run(Settings {
        default_font: Font {
            family: Family::Name("Noto Sans CJK HK"),
            ..Default::default()
        },
        ..Default::default()
    })?;

    Ok(())
}

struct ExampleApp;

impl Sandbox for ExampleApp {
    type Message = ();

    fn new() -> Self {
        ExampleApp
    }

    fn title(&self) -> String {
        String::from("Example App")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let font_bold = Font {
            weight: Weight::Bold,
            family: Family::Name("Noto Sans CJK HK"),
            ..Default::default()
        };
        let big_size = 24;
        let small_size = 16;
        widget::container(widget::container(widget::row!(
            widget::image(image::Handle::from_path("assets/ushio_noa/ushio_noa.png")).width(Length::Fixed(232.0)).height(Length::Fixed(420.0)),
            widget::column!(
                widget::text("USHIO NOA").size(big_size).font(font_bold),
                widget::text("生盐诺亚").size(big_size).font(font_bold),
                widget::vertical_space(Length::Fixed(10.0)),
                widget::text("声优: 鈴代紗弓").size(small_size).font(font_bold),
                widget::text("设计: Hwansang").size(small_size).font(font_bold),
                widget::text("原画: DoReMi").size(small_size).font(font_bold),
            ).spacing(5),
            widget::column({
                let mut column = Vec::with_capacity(8);
                column.push(
                    widget::container(widget::text("档案").size(big_size).font(font_bold))
                        .padding(Padding::from(5)).style(theme::Container::Custom(Box::new(ContainerTitleStyle))).into()
                );
                column.push(widget::vertical_space(5.0).into());
                for text in ["所属: 千年科学学校", "社团: 研讨会", "生日: 4月13日", "学年: 2年级", "年龄: 16岁", "身高: 161cm", "兴趣: 读书、背诵"] {
                    column.push(widget::text(text).size(small_size).font(font_bold).into());
                }
                column
            }).spacing(5),
            widget::column!(
                widget::container(widget::text("基本情报").size(big_size).font(font_bold))
                    .padding(Padding::from(5)).style(theme::Container::Custom(Box::new(ContainerTitleStyle))),
                widget::vertical_space(5.0),
                widget::text("千年科学学校所属，学生会「研讨会」的书记。").size(small_size),
                widget::text("主要负责记录研讨会决定的事项和会议记录。还负责对千禧的学生们发明的产品的专利进行鉴别并登记的律师业务。诺亚拥有非常出色的记忆力，只要是她见过的听到过的内容她都能完整的记下来。").size(small_size),
            ).width(Length::Fixed(250.0)).spacing(5.0)
        ).spacing(25))
            .padding(Padding::from(25))
            .style(theme::Container::Custom(Box::new(ContainerBoxStyle)))
        )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(Vertical::Top)
            .center_x()
            .into()
    }
}

struct ContainerTitleStyle;

impl container::StyleSheet for ContainerTitleStyle {
    type Style = theme::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border_width: 4.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }
}

struct ContainerBoxStyle;

impl container::StyleSheet for ContainerBoxStyle {
    type Style = theme::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border_radius: BorderRadius::from(10.0),
            border_width: 20.0,
            border_color: iced::color!(198, 233, 255),
            ..Default::default()
        }
    }
}
