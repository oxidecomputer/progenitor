pub mod buildomat_builder;
pub mod buildomat_builder_tagged;
pub mod buildomat_cli;
//pub mod buildomat_httpmock;
pub mod buildomat_positional;
pub mod keeper_builder;
pub mod keeper_builder_tagged;
pub mod keeper_cli;
pub mod keeper_httpmock;
pub mod keeper_positional;
pub mod nexus_builder;
pub mod nexus_builder_tagged;
pub mod nexus_cli;
pub mod nexus_httpmock;
pub mod nexus_positional;
pub mod param_collision_builder;
pub mod param_collision_builder_tagged;
pub mod param_collision_cli;
pub mod param_collision_httpmock;
pub mod param_collision_positional;
pub mod param_overrides_builder;
pub mod param_overrides_builder_tagged;
pub mod param_overrides_cli;
pub mod param_overrides_httpmock;
pub mod param_overrides_positional;
pub mod propolis_server_builder;
pub mod propolis_server_builder_tagged;
pub mod propolis_server_cli;
//pub mod propolis_server_httpmock;
pub mod propolis_server_positional;
pub mod test_default_params_builder;
pub mod test_default_params_positional;
pub mod test_freeform_response;
pub mod test_renamed_parameters;

// progenitor emits `clap::value_parser!(T)` for every argument, and clap
// infers the parser from the traits the type implements. `GetThingOrThingsId`
// implements none that clap accepts, so we, as the consumer, supply the
// parser through `ValueParserFactory`, which is what the CLI generator asks a
// consumer to do.
impl ::clap::builder::ValueParserFactory for buildomat_builder::types::GetThingOrThingsId {
    type Parser = ::clap::builder::MapValueParser<
        ::clap::builder::StringValueParser,
        fn(String) -> buildomat_builder::types::GetThingOrThingsId,
    >;

    fn value_parser() -> Self::Parser {
        ::clap::builder::TypedValueParser::map(
            ::clap::builder::StringValueParser::new(),
            buildomat_builder::types::GetThingOrThingsId::String
                as fn(String) -> buildomat_builder::types::GetThingOrThingsId,
        )
    }
}
