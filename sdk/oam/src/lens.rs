// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_attached_links_output_next_token(
    input: &crate::output::ListAttachedLinksOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_links_output_next_token(
    input: &crate::output::ListLinksOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_sinks_output_next_token(
    input: &crate::output::ListSinksOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_attached_links_output_items(
    input: crate::output::ListAttachedLinksOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ListAttachedLinksItem>> {
    let input = match input.items {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_links_output_items(
    input: crate::output::ListLinksOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ListLinksItem>> {
    let input = match input.items {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_sinks_output_items(
    input: crate::output::ListSinksOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ListSinksItem>> {
    let input = match input.items {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
