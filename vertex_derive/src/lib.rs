#![recursion_limit = "128"]

extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

#[proc_macro_derive(VertexAttribPointers, attributes(location, divisor))]
pub fn vertex_attrib_pointers_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = generate_impl(&ast);
    gen.parse().unwrap()
}

fn generate_impl(ast: &syn::DeriveInput) -> quote::Tokens {
    let ident = &ast.ident;
    let generics = &ast.generics;
    let where_clause = &ast.generics.where_clause;
    let fields_vertex_attrib_pointer = generate_vertex_attrib_pointer_calls(&ast.body);

    let vertex_buffer = syn::Ident::from(ident.to_string() + &"Buffer".to_string());
    // let vertex_buffer = syn::Ident::from("VertexBuffer".to_string());
        quote!{
            pub struct #vertex_buffer<T> {
                vbo: red::buffer::ArrayBuffer,
                pub vao: red::buffer::VertexArray,
                pub len: Option<usize>, // a bit ugly
                _phantom_data: std::marker::PhantomData<T>
            }

            impl #vertex_buffer<#ident> {
                pub fn empty_new(gl: &red::GL) -> Result<#vertex_buffer<#ident>, String> {
                    let vbo: red::buffer::Buffer<red::buffer::BufferTypeArray> 
                        = red::buffer::Buffer::new(&gl)?;
                    Ok(#vertex_buffer {
                        vbo: vbo,
                        vao: red::buffer::VertexArray::new(gl)?,
                        len: None, // DO SOMETHING WITH THAT
                        _phantom_data: std::marker::PhantomData
                    })
                }

                pub fn dynamic_draw_data(&mut self, shape: &[#ident]) {
                    self.vbo.bind();
                    self.vbo.dynamic_draw_data(shape);
                    self.len = Some(shape.len());
                    self.vbo.unbind();
                }

                pub fn new(gl: &red::GL, shape: &[#ident]) -> Result<#vertex_buffer<#ident>, String> {
                    let vbo: red::buffer::Buffer<red::buffer::BufferTypeArray> 
                        = red::buffer::Buffer::new(&gl)?;
                    vbo.bind();
                    vbo.static_draw_data(shape);
                    vbo.unbind();
                    Ok(#vertex_buffer {
                        vbo: vbo,
                        vao: red::buffer::VertexArray::new(gl)?,
                        len: Some(shape.len()),
                        _phantom_data: std::marker::PhantomData
                    })
                }

                pub fn map_array(&self) -> Result<red::buffer::MapArray<#ident>, String>{
                    unsafe {
                        self.vbo.bind();
                        let res: Result<red::buffer::MapArray<#ident>, String> = 
                            red::buffer::MapArray::new(&self.vbo.gl, self.len.unwrap()); // TODO: move gl to vertex buffer
                        res
                    }
                }

            }

            impl red::buffer::VertexBufferBehavior for #vertex_buffer<#ident> {
                #[allow(unused_variables)]
                fn vertex_attrib_pointers(&self, gl: &red::GL, program: &red::shader::Program) {
                    let stride = ::std::mem::size_of::<#ident>();
                    let offset = 0;

                    #(#fields_vertex_attrib_pointer)*
                }

                fn bind(&self) {
                    self.vbo.bind();
                }

                fn unbind(&self) {
                    self.vbo.unbind();
                }

            }
        }
    // );
}

fn generate_vertex_attrib_pointer_calls(body: &syn::Body) -> Vec<quote::Tokens> {
    match body {
        &syn::Body::Enum(_) => panic!("VertexAttribPointers can not be implemented for enums"),
        &syn::Body::Struct(syn::VariantData::Unit) => {
            panic!("VertexAttribPointers can not be implemented for Unit structs")
        }
        &syn::Body::Struct(syn::VariantData::Tuple(_)) => {
            panic!("VertexAttribPointers can not be implemented for Tuple structs")
        }
        &syn::Body::Struct(syn::VariantData::Struct(ref s)) => s
            .iter()
            .map(generate_struct_field_vertex_attrib_pointer_call)
            .collect(),
    }
}

fn generate_struct_field_vertex_attrib_pointer_call(field: &syn::Field) -> quote::Tokens {
    let field_name = match field.ident {
        Some(ref i) => format!("{}", i),
        None => String::from(""),
    };
    let field_ty = &field.ty;
        let divisor_call = match field
        .attrs
        .iter()
        .filter(|a| a.value.name() == "divisor")
        .next()
    {
        Some(attr) => {
            let divisor_value: u32 = match attr.value {
                syn::MetaItem::NameValue(_, syn::Lit::Str(ref s, _)) => {
                    s.parse().unwrap_or_else(|_| {
                        panic!(
                            "Field {} divisor attribute value must contain an integer",
                            field_name
                        )
                    })
                }
                _ => panic!(
                    "Field {} divisor attribute value must be a string literal",
                    field_name
                ),
            };

            quote! {
                gl.vertex_attrib_divisor(location, #divisor_value);
            }
        }
        None => quote!{},
    };
    
    quote! {
        let location = unsafe {
            gl.get_attrib_location(program.id(), &#field_name)
        };
        if location < 0 {
            panic!("vertex attribute {} is not found in shader or is not active", #field_name);
        }
        let location = location as u32;
        unsafe {
            #field_ty::vertex_attrib_pointer(gl, stride as ::std::os::raw::c_int, location, offset as ::std::os::raw::c_int);
            // #divisor_call
            #divisor_call
        }
        let offset = offset + ::std::mem::size_of::<#field_ty>();
    }
}