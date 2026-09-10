use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

/// Resolve the GPUI API exposed to the crate where a macro is expanded.
///
/// `gpui-kit` is preferred because it re-exports GPUI and is the only direct
/// dependency required by kit consumers. Standalone consumers may use either
/// the published `gpui-pre` snapshot or Zed's `gpui` package, including aliases.
pub(crate) fn gpui() -> syn::Result<TokenStream> {
    for name in ["gpui-kit", "gpui-pre", "gpui"] {
        if let Ok(found) = crate_name(name) {
            return Ok(found_crate_path(found));
        }
    }
    Err(syn::Error::new(
        Span::call_site(),
        "IntoPlot requires a direct dependency on `gpui-kit`, `gpui-pre`, or `gpui`",
    ))
}

fn found_crate_path(found: FoundCrate) -> TokenStream {
    match found {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!(::#ident)
        }
    }
}
