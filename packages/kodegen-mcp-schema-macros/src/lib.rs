use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse::{Parse, ParseStream}, ItemImpl, Lit, Meta, Expr, Token};

// Custom parser for attribute arguments
struct ToolMetadataArgs {
    name: String,
    category: String,
    description: String,
}

impl Parse for ToolMetadataArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut category = None;
        let mut description = None;

        // Parse comma-separated name=value pairs
        while !input.is_empty() {
            let meta: Meta = input.parse()?;
            
            if let Meta::NameValue(nv) = meta {
                let ident = nv.path.get_ident()
                    .ok_or_else(|| syn::Error::new_spanned(&nv.path, "Expected identifier"))?
                    .to_string();
                    
                if let Expr::Lit(expr_lit) = &nv.value {
                    if let Lit::Str(lit) = &expr_lit.lit {
                        match ident.as_str() {
                            "name" => name = Some(lit.value()),
                            "category" => category = Some(lit.value()),
                            "description" => description = Some(lit.value()),
                            _ => return Err(syn::Error::new_spanned(&nv.path, "Unknown attribute")),
                        }
                    } else {
                        return Err(syn::Error::new_spanned(&nv.value, "Expected string literal"));
                    }
                } else {
                    return Err(syn::Error::new_spanned(&nv.value, "Expected string literal"));
                }
            } else {
                return Err(syn::Error::new_spanned(meta, "Expected name=value pair"));
            }

            // Parse optional comma
            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            }
        }

        Ok(ToolMetadataArgs {
            name: name.ok_or_else(|| input.error("Missing 'name' attribute"))?,
            category: category.ok_or_else(|| input.error("Missing 'category' attribute"))?,
            description: description.ok_or_else(|| input.error("Missing 'description' attribute"))?,
        })
    }
}

#[proc_macro_attribute]
pub fn tool_metadata(attr: TokenStream, item: TokenStream) -> TokenStream {
    let impl_block = parse_macro_input!(item as ItemImpl);
    let args = parse_macro_input!(attr as ToolMetadataArgs);

    let name = args.name;
    let category = args.category;
    let description = args.description;

    let self_ty = &impl_block.self_ty;

    // Extract Output and Prompts types from impl block
    let mut output_ty = None;
    let mut prompts_ty = None;

    for item in &impl_block.items {
        if let syn::ImplItem::Type(ty) = item {
            if ty.ident == "Output" {
                output_ty = Some(&ty.ty);
            } else if ty.ident == "Prompts" {
                prompts_ty = Some(&ty.ty);
            }
        }
    }

    let output_ty = output_ty.expect("ToolArgs impl must have Output type");
    let prompts_ty = prompts_ty.expect("ToolArgs impl must have Prompts type");

    // Generate inventory registration
    let expanded = quote! {
        #impl_block

        // Auto-generated inventory registration
        inventory::submit! {
            crate::ToolMetadata {
                name: #name,
                category: #category,
                description: #description,
                args_schema: || {
                    let schema = schemars::schema_for!(#self_ty);
                    serde_json::to_value(&schema).expect("Failed to serialize schema")
                },
                output_schema: || {
                    let schema = schemars::schema_for!(#output_ty);
                    serde_json::to_value(&schema).expect("Failed to serialize schema")
                },
                prompt_arguments: || {
                    <#prompts_ty as crate::tool::PromptProvider>::prompt_arguments()
                },
                generate_prompts: |args_json: &serde_json::Value| {
                    // Deserialize JSON to the prompt's PromptArgs type
                    // Fallback to empty object if provided args fail to deserialize
                    // (works because all PromptArgs fields use #[serde(default)])
                    let args: <#prompts_ty as crate::tool::PromptProvider>::PromptArgs = 
                        serde_json::from_value(args_json.clone())
                            .or_else(|_| serde_json::from_value(serde_json::json!({})))
                            .expect("PromptArgs should deserialize from empty object");
                    // Call the actual generate_prompts implementation
                    <#prompts_ty as crate::tool::PromptProvider>::generate_prompts(&args)
                },
            }
        }
    };

    TokenStream::from(expanded)
}
