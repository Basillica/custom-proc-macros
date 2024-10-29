extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};
use quote::{ToTokens, quote};
use syn::spanned::Spanned;


#[proc_macro_derive(FactoryPattern, attributes(factory))]
pub fn derive_factory_pattern(input: TokenStream) -> TokenStream {
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


#[proc_macro_derive(BuiderPattern)]
pub fn derive_builder_pattern(input: TokenStream) -> TokenStream {
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


#[proc_macro_derive(DeriveGetterSetter)]
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

                let getter_name = syn::Ident::new(&format!("get_{}", field_name), field_name.span());
                // Generate a getter method for each field
                let getter_fn = quote! {
                    impl #struct_name {
                        pub fn #getter_name(&self) -> &#field_ty {
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


#[proc_macro_derive(SqlQueryDerive2)]
pub fn sql_query_buider2(input: TokenStream) -> TokenStream {
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


#[proc_macro_derive(SqlQueryDerive, attributes(table_name, use_attrs_with_query))]
pub fn sql_query_buider(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    

    // Get the struct's name
    let name = input.ident;

    // Default values for table name and extra derives
    let mut table_name = "".to_string(); // Default table name
    let mut use_attrs_with_query = false;

    for attr in &input.attrs {
        if let Some(attr_meta_name) = attr.path().get_ident() {
            // match table name
            if attr_meta_name == "table_name" {
                let exp: syn::Expr = attr
                    .parse_args()
                    .map_err(|_| {
                        // returning a specific syn::Error to teach the right usage of your macro 
                        syn::Error::new(
                            attr.span(),
                            // this indoc macro is just convenience and requires the indoc crate but can be done without it
                            format! {r#"
                                The `table_name` attribute expects string literals to be comma separated

                                = help: use `#[table_name("users")]`
                            "#}
                        )
                    }).unwrap();
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