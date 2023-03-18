use comrak::plugins::syntect::SyntectAdapter;
use comrak::{markdown_to_html_with_plugins, ComrakExtensionOptions, ComrakOptions, ComrakPlugins};

pub fn convert_to_html(md: &str) -> String {
    let options = ComrakOptions {
        extension: ComrakExtensionOptions {
            strikethrough: true,
            tagfilter: true,
            table: true,
            autolink: true,
            tasklist: true,
            superscript: true,
            header_ids: None,
            footnotes: true,
            description_lists: true,
            front_matter_delimiter: None,
            shortcodes: true,
        },
        parse: Default::default(),
        render: Default::default(),
    };
    let mut plugins = ComrakPlugins::default();

    let adapter = SyntectAdapter::new("base16-ocean.dark");
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    markdown_to_html_with_plugins(md, &options, &plugins)
}
