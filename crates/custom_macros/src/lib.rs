extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Meta, Lit, Ident, Token};
use quote::{ToTokens, quote};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::parse::{Parse, ParseStream};


#[proc_macro_derive(Factory, attributes(factory))]
pub fn derive_factory(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the struct name
    let struct_name = input.ident;
    let factory_name = Ident::new(&format!("new_{}", struct_name.to_string().to_lowercase()), struct_name.span());

    // Generate code only if the input is a struct
    let expanded = if let Data::Struct(data) = input.data {
        let mut constructor_params = vec![];
        let mut constructor_fields = vec![];

        // Loop through each field in the struct
        if let Fields::Named(fields) = data.fields {
            for field in fields.named {
                let field_name = field.ident.as_ref().unwrap();
                let field_ty = &field.ty;

                // Check for #[factory] attribute
                let mut is_factory_default = false;

                for attr in &field.attrs {
                    if let Some(attr_meta_name) = attr.path().get_ident() {
                        if attr_meta_name == "factory" {
                            is_factory_default = true;
                        }
                    }
                    // if let Ok(Meta::Path(path)) = attr.parse_args() {
                    //     if path.is_ident("factory") {
                    //         is_factory_default = true;
                    //     }
                    // }
                }

                // If the field is marked with #[factory], don't require it as a parameter in the factory method
                if is_factory_default {
                    constructor_fields.push(quote! {
                        #field_name: Default::default()
                    });
                } else {
                    // If not marked with #[factory], require it as a parameter
                    constructor_params.push(quote! {
                        #field_name: #field_ty
                    });
                    constructor_fields.push(quote! {
                        #field_name
                    });
                }
            }
        }

        // Generate the factory method
        let factory_method = quote! {
            impl #struct_name {
                pub fn #factory_name(#(#constructor_params),*) -> Self {
                    Self {
                        #(#constructor_fields),*
                    }
                }
            }
        };

        factory_method
    } else {
        // Return an empty TokenStream if it's not a struct
        quote! {}
    };

    // Convert the expanded code into a TokenStream
    TokenStream::from(expanded)
}


#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the struct name
    let struct_name = input.ident;
    let builder_name = Ident::new(&format!("{}Builder", struct_name), struct_name.span());

    // Generate code only if the input is a struct
    let expanded = if let Data::Struct(data) = input.data {
        let mut builder_fields = vec![];
        let mut builder_methods = vec![];
        let mut build_fields = vec![];
        let mut constructor_params = vec![];

        // Loop through each field in the struct
        if let Fields::Named(fields) = data.fields {
            for field in fields.named {
                let field_name = field.ident.as_ref().unwrap();
                let field_ty = &field.ty;

                // Builder struct will have Option types for each field
                builder_fields.push(quote! {
                    #field_name: std::option::Option<#field_ty>
                });

                // Builder methods (chained methods)
                builder_methods.push(quote! {
                    pub fn #field_name(&mut self, #field_name: #field_ty) -> &mut Self {
                        self.#field_name = std::option::Option::Some(#field_name);
                        self
                    }
                });

                // In the build function, unwrap fields to get their values
                build_fields.push(quote! {
                    #field_name: self.#field_name.take().expect(concat!(stringify!(#field_name), " is not set"))
                });

                // Default constructor params to None
                constructor_params.push(quote! {
                    #field_name: std::option::Option::None
                });
            }
        }

        // Generate the `build` method and builder struct
        let builder_struct = quote! {
            pub struct #builder_name {
                #(#builder_fields),*
            }

            impl #builder_name {
                // Builder methods
                #(#builder_methods)*

                // Build method to create the struct
                pub fn build(&mut self) -> std::result::Result<#struct_name, &'static str> {
                    Ok(#struct_name {
                        #(#build_fields),*
                    })
                }
            }

            impl #struct_name {
                // Method to create a new builder instance
                pub fn builder() -> #builder_name {
                    #builder_name {
                        #(#constructor_params),*
                    }
                }
            }
        };

        builder_struct
    } else {
        // Return an empty TokenStream if it's not a struct
        quote! {}
    };

    // Convert the expanded code into a TokenStream
    TokenStream::from(expanded)
}


#[proc_macro_derive(GettersSetters)]
pub fn derive_getters_setters(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the struct name
    let struct_name = input.ident;

    // Generate code only if the input is a struct
    let expanded = if let Data::Struct(data) = input.data {
        let mut getter_setters = vec![];
        let mut constructor_fields = vec![];
        let mut constructor_params = vec![];

        // Loop through each field in the struct
        if let Fields::Named(fields) = data.fields {
            for field in fields.named {
                let field_name = field.ident.as_ref().unwrap();
                let field_ty = &field.ty;

                // Generate a getter method for each field
                let getter_fn = quote! {
                    impl #struct_name {
                        pub fn #field_name(&self) -> &#field_ty {
                            &self.#field_name
                        }
                    }
                };
                getter_setters.push(getter_fn);

                // Generate the setter method name dynamically
                let setter_name = Ident::new(&format!("set_{}", field_name), field_name.span());

                // Generate a setter method for each field
                let setter_fn = quote! {
                    impl #struct_name {
                        pub fn #setter_name(&mut self, value: #field_ty) {
                            self.#field_name = value;
                        }
                    }
                };
                getter_setters.push(setter_fn);

                // Collect the fields and parameters for the new function
                constructor_fields.push(quote! { #field_name });
                constructor_params.push(quote! { #field_name: #field_ty });
            }
        }

        // Generate the `new` constructor
        let new_fn = quote! {
            impl #struct_name {
                pub fn new(#(#constructor_params),*) -> Self {
                    Self {
                        #(#constructor_fields),*
                    }
                }
            }
        };
        
        // Combine the generated getters, setters, and new function
        quote! {
            #(#getter_setters)*
            #new_fn
        }
    } else {
        // Return an empty TokenStream if it's not a struct
        quote! {}
    };

    // Convert the expanded code into a TokenStream
    TokenStream::from(expanded)
}


#[proc_macro_derive(QueryBuilder2)]
pub fn query_builder_derive_v1(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the struct's name
    let name = input.ident;

    // Check if the struct has named fields
    let fields = match input.data {
        Data::Struct(ref data_struct) => match data_struct.fields {
            Fields::Named(ref fields_named) => &fields_named.named,
            _ => panic!("QueryBuilder can only be derived for structs with named fields"),
        },
        _ => panic!("QueryBuilder can only be derived for structs"),
    };

    // Generate SQL field checks for each field in the struct
    let query_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_name_str = field_name.as_ref().unwrap().to_string();

        // For each field, check if it is Some() and then include it in the query
        quote! {
            if let Some(ref value) = self.#field_name {
                query_parts.push(format!("{} = '{}'", #field_name_str, value));
            }
        }
    });

    // Generate the final query construction logic
    let expanded = quote! {
        impl #name {
            pub fn build_query(&self) -> String {
                let mut query_parts = Vec::new();

                #(#query_fields)*

                if query_parts.is_empty() {
                    return "SELECT * FROM table_name".to_string();
                }

                let where_clause = query_parts.join(" AND ");
                format!("SELECT * FROM table_name WHERE {}", where_clause)
            }
        }
    };

    // Convert the generated code into a TokenStream and return it
    TokenStream::from(expanded)
}


// Custom struct to represent parsed custom_model attributes
struct CustomModel {
    name: Ident,
    fields: Vec<Ident>,
    extra_derives: Vec<Ident>,
}


// Implementing the Parse trait to parse the custom_model attribute input
impl Parse for CustomModel {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut fields = Vec::new();
        let mut extra_derives = Vec::new();

        // Parse the input for named items like name, fields, and extra_derives
        while !input.is_empty() {
            let lookahead = input.lookahead1();

            if lookahead.peek(Ident) {
                let key: Ident = input.parse()?;
                input.parse::<Token![=]>()?;
                if key == "name" {
                    let lit: Lit = input.parse()?;
                    if let Lit::Str(lit_str) = lit {
                        name = Some(Ident::new(&lit_str.value(), lit_str.span()));
                    }
                } else if key == "fields" {
                    let content;
                    syn::bracketed!(content in input);
                    fields = Punctuated::<Ident, Token![,]>::parse_terminated(&content)?
                        .into_iter()
                        .collect();
                } else if key == "extra_derives" {
                    let content;
                    syn::bracketed!(content in input);
                    extra_derives = Punctuated::<Ident, Token![,]>::parse_terminated(&content)?
                        .into_iter()
                        .collect();
                }
            } else {
                return Err(syn::Error::new(input.span(), "unexpected input"));
            }

            // Optionally accept commas
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(CustomModel {
            name: name.expect("model must have a name"),
            fields,
            extra_derives,
        })
    }
}


#[proc_macro_derive(QueryBuilder, attributes(custom_model))]
pub fn query_builder_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Get the struct's name
    let name = input.ident;

    // Extract and process custom_model attributes
    let mut models = vec![];
    for attr in input.attrs.iter().filter(|attr| attr.path().is_ident("custom_model")) {
        if let Ok(meta) = attr.parse_args() {
            if let Meta::List(meta_list) = meta {
                for token in meta_list.tokens {
                    // Parse the custom model attribute
                    let parsed: Result<CustomModel, _> = syn::parse2(token.to_token_stream());
                    if let Ok(custom_model) = parsed {
                        models.push(custom_model);
                    }
                }
            }
        }
    }
    

    // Generate SQL query builders for each custom model
    let model_impls = models.into_iter().map(|model| {
        let model_name = model.name;
        let model_fields = model.fields;
        let extra_derives = model.extra_derives;

        // Check if the struct has named fields
        let fields = match input.data {
            Data::Struct(ref data_struct) => match data_struct.fields {
                Fields::Named(ref fields_named) => &fields_named.named,
                _ => panic!("QueryBuilder can only be derived for structs with named fields"),
            },
            _ => panic!("QueryBuilder can only be derived for structs"),
        };

        // Generate SQL field checks for each field in the struct
        let query_fields = fields.iter().map(|field| {
            let field_name = &field.ident;
            let field_name_str = field_name.as_ref().unwrap().to_string();

            // For each field, check if it is Some() and then include it in the query
            quote! {
                if let Some(ref value) = self.#field_name {
                    query_parts.push(format!("{} = '{}'", #field_name_str, value));
                }
            }
        });

        quote! {
            impl #name {
                pub fn build_query(&self) -> String {
                    let mut query_parts = Vec::new();

                    #(#query_fields)*

                    if query_parts.is_empty() {
                        return format!("SELECT * FROM {}", stringify!(#model_name));
                    }

                    let where_clause = query_parts.join(" AND ");
                    format!("SELECT * FROM {} WHERE {}", stringify!(#model_name), where_clause)
                }
            }

            #[derive(#(#extra_derives),*)]
            pub struct #name {
                #(#model_fields: Option<String>),*
            }
        }
    });

    // Combine the generated implementations
    let expanded = quote! {
        #(#model_impls)*
    };

    TokenStream::from(expanded)
}


struct MyParser {
    v: Vec<String>,
}


impl Parse for MyParser {
    #[inline]
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut v: Vec<String> = vec![];

        loop {
            if input.is_empty() {
                break;
            }

            v.push(input.parse::<syn::LitStr>()?.value());

            if input.is_empty() {
                break;
            }

            input.parse::<Token!(,)>()?;
        }

        Ok(MyParser {
            v,
        })
    }
}


#[proc_macro_derive(QueryBuilder3, attributes(table_name, query_params, use_attrs_with_query))]
pub fn query_builder_derive_v3(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    

    // Get the struct's name
    let name = input.ident;

    // Default values for table name and extra derives
    let mut table_name = "".to_string(); // Default table name
    let mut use_attrs_with_query = false;
    let mut params: Vec<String> = Vec::new();


    for attr in &input.attrs {
        if let Some(attr_meta_name) = attr.path().get_ident() {

            // match derives
            if attr_meta_name == "query_params" {
                let attr_meta = &attr.meta;
                match attr_meta {
                    Meta::List(list) => {
                        let parsed: MyParser = list.parse_args()
                        .map_err(|_| {
                            // returning a specific syn::Error to teach the right usage of your macro 
                            syn::Error::new(
                                list.span(),
                                // this indoc macro is just convenience and requires the indoc crate but can be done without it
                                format! {r#"
                                    The `query_params` attribute expects string literals to be comma separated
    
                                    = help: use `#[query_params("Debug", "Clone")]`
                                "#}
                            )
                        }).unwrap();
                        
                        params.extend_from_slice(&parsed.v);
                    },
                    _ => panic!("Incorrect format for using the `hello` attribute."),
                }
            }

            // match table name
            if attr_meta_name == "table_name" {
                let exp: syn::Expr = attr
                    .parse_args()
                    .unwrap();
                table_name = exp.into_token_stream().to_string();
            }

            // match attrs with query
            if attr_meta_name == "use_attrs_with_query" {
                use_attrs_with_query = true
            }
        }
    }

    // Check if the struct has named fields
    let fields = match input.data {
        Data::Struct(ref data_struct) => match data_struct.fields {
            Fields::Named(ref fields_named) => &fields_named.named,
            _ => panic!("QueryBuilder can only be derived for structs with named fields"),
        },
        _ => panic!("QueryBuilder can only be derived for structs"),
    };

    // Generate SQL field checks for each field in the struct
    let query_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_name_str = field_name.as_ref().unwrap().to_string();

        // For each field, check if it is Some() and then include it in the query
        quote! {
            if let Some(ref value) = self.#field_name {
                query_parts.push(format!("{} = '{}'", #field_name_str, value));
            }
        }
    });

    // Generate the final query construction logic
    let expanded = quote! {

        impl #name {
            pub fn build_query(&self) -> String {
                
                let mut query_parts = Vec::new();

                #(#query_fields)*

                if query_parts.is_empty() {
                    return format!("SELECT * FROM {}", #table_name);
                }

                let result: String = query_parts.iter()
                    .map(|item| {
                        // Split the string by the `=` character
                        let parts: Vec<&str> = item.split('=').collect();
                        // Extract and trim the key (left side of `=`)
                        parts[0].trim()
                    })
                    .collect::<Vec<&str>>()
                    .join(", ");


                let where_clause = query_parts.join(" AND ");
                if #use_attrs_with_query {
                    return format!("SELECT {:?} FROM {} WHERE {}", result.replace("\"", ""),  #table_name, where_clause)
                }
                
                format!("SELECT * FROM {} WHERE {}", #table_name, where_clause)
            }
        }
    
    };

    // Convert the generated code into a TokenStream and return it
    TokenStream::from(expanded)
}