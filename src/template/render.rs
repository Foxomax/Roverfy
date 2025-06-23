use crate::http::{HttpResponse, StatusCode, ContentType};
use crate::template::TEMPLATE_ENGINE;
use std::collections::HashMap;
use tera::{Context, Error as TeraError};

/// Renders a template with the given context and returns an HTTP response.
/// 
/// This function takes a template path and a context map, renders the template
/// using the Tera template engine, and returns the result as an HTTP response
/// with proper HTML content type.
/// 
/// # Arguments
/// 
/// * `template_path` - The path to the template file (relative to templates directory)
/// * `context` - A HashMap containing key-value pairs for template variables
/// 
/// # Returns
/// 
/// A `Result` containing either:
/// * `Ok(Vec<u8>)` - The rendered HTTP response as bytes
/// * `Err(TeraError)` - Template rendering error
/// 
/// # Examples
/// 
/// ```
/// # use std::collections::HashMap;
/// # use roverfy::template::render;
/// # roverfy::config::init_test_config();
/// # 
/// let mut context = HashMap::new();
/// context.insert("name".to_string(), "John".to_string());
/// context.insert("title".to_string(), "Welcome".to_string());
/// 
/// match render("test/index.html".to_string(), &context) {
///     Ok(response_bytes) => {
///         // Send response_bytes to client
///         println!("Template rendered successfully");
///     },
///     Err(e) => {
///         eprintln!("Template rendering failed: {}", e);
///     }
/// }
/// ```
/// 
/// # Template Example
/// 
/// If you have a template file `templates/index.html`:
/// ```html
/// <!DOCTYPE html>
/// <html>
/// <head>
///     <title>{{ title }}</title>
/// </head>
/// <body>
///     <h1>Hello, {{ name }}!</h1>
///     <p>Welcome to our website.</p>
/// </body>
/// </html>
/// ```
/// 
/// The rendered output will be a complete HTTP response with the HTML content.
pub fn render(
    template_path: String,
    context: &HashMap<String, String>,
) -> Result<Vec<u8>, TeraError> {
    // Convert HashMap to Tera Context
    let tera_context = Context::from_serialize(context)
        .map_err(|e| TeraError::from(format!("Failed to serialize context: {}", e)))?;
    
    // Render the template
    let rendered_content = TEMPLATE_ENGINE.render(&template_path, &tera_context)?;
    
    // Create HTTP response with HTML content type
    let response = HttpResponse::html(StatusCode::OK, &rendered_content);
    
    // Convert to bytes
    Ok(response.to_bytes())
}

/// Renders a template with the given context and returns an HTTP response with custom status code.
/// 
/// This function is similar to `render()` but allows you to specify a custom HTTP status code.
/// 
/// # Arguments
/// 
/// * `template_path` - The path to the template file
/// * `context` - A HashMap containing key-value pairs for template variables
/// * `status_code` - The HTTP status code for the response
/// 
/// # Returns
/// 
/// A `Result` containing either the rendered HTTP response as bytes or a template error.
/// 
/// # Examples
/// 
/// ```
/// # use std::collections::HashMap;
/// # use roverfy::http::StatusCode;
/// # use roverfy::template::render_with_status;
/// # roverfy::config::init_test_config();
/// # 
/// let mut context = HashMap::new();
/// context.insert("error_message".to_string(), "Page not found".to_string());
/// 
/// let response = render_with_status(
///     "test/index.html".to_string(),
///     &context,
///     StatusCode::NotFound
/// );
/// ```
pub fn render_with_status(
    template_path: String,
    context: &HashMap<String, String>,
    status_code: StatusCode,
) -> Result<Vec<u8>, TeraError> {
    // Convert HashMap to Tera Context
    let tera_context = Context::from_serialize(context)
        .map_err(|e| TeraError::from(format!("Failed to serialize context: {}", e)))?;
    
    // Render the template
    let rendered_content = TEMPLATE_ENGINE.render(&template_path, &tera_context)?;
    
    // Create HTTP response with custom status code
    let mut response = HttpResponse::new(status_code, vec![], rendered_content);
    response.add_header("Content-Type", ContentType::Html.as_str());
    
    // Convert to bytes
    Ok(response.to_bytes())
}

/// Renders a template and returns the raw HTML content without HTTP headers.
/// 
/// This function is useful when you need just the rendered HTML content
/// without the HTTP response wrapper.
/// 
/// # Arguments
/// 
/// * `template_path` - The path to the template file
/// * `context` - A HashMap containing key-value pairs for template variables
/// 
/// # Returns
/// 
/// A `Result` containing either the rendered HTML string or a template error.
/// 
/// # Examples
/// 
/// ```
/// # use std::collections::HashMap;
/// # use roverfy::template::render_html;
/// # roverfy::config::init_test_config();
/// # 
/// let mut context = HashMap::new();
/// context.insert("name".to_string(), "Alice".to_string());
/// 
/// match render_html("test/index.html".to_string(), &context) {
///     Ok(html) => {
///         println!("Rendered HTML: {}", html);
///     },
///     Err(e) => {
///         eprintln!("Rendering failed: {}", e);
///     }
/// }
/// ```
pub fn render_html(
    template_path: String,
    context: &HashMap<String, String>,
) -> Result<String, TeraError> {
    // Convert HashMap to Tera Context
    let tera_context = Context::from_serialize(context)
        .map_err(|e| TeraError::from(format!("Failed to serialize context: {}", e)))?;
    
    // Render the template and return the HTML string
    TEMPLATE_ENGINE.render(&template_path, &tera_context)
}

/// Renders a template with error handling and returns a fallback response on failure.
/// 
/// This function provides a safe way to render templates with automatic fallback
/// to an error page if the template rendering fails.
/// 
/// # Arguments
/// 
/// * `template_path` - The path to the template file
/// * `context` - A HashMap containing key-value pairs for template variables
/// * `fallback_message` - The message to display if template rendering fails
/// 
/// # Returns
/// 
/// Always returns a valid HTTP response. If template rendering succeeds,
/// returns the rendered template. If it fails, returns a simple error page.
/// 
/// # Examples
/// 
/// ```
/// # use std::collections::HashMap;
/// # use roverfy::template::render_safe;
/// # roverfy::config::init_test_config();
/// # 
/// let mut context = HashMap::new();
/// context.insert("user_name".to_string(), "Bob".to_string());
/// 
/// let response_bytes = render_safe(
///     "test/index.html".to_string(),
///     &context,
///     "Unable to load profile page"
/// );
/// 
/// // response_bytes is always valid, even if template rendering failed
/// ```
pub fn render_safe(
    template_path: String,
    context: &HashMap<String, String>,
    fallback_message: &str,
) -> Vec<u8> {
    match render(template_path, context) {
        Ok(response_bytes) => response_bytes,
        Err(_) => {
            // Create a simple error response if template rendering fails
            let error_html = format!(
                r#"<!DOCTYPE html>
<html>
<head>
    <title>Error</title>
    <meta charset="UTF-8">
</head>
<body>
    <h1>Error</h1>
    <p>{}</p>
    <p>Please try again later.</p>
</body>
</html>"#,
                fallback_message
            );
            
            let response = HttpResponse::html(StatusCode::InternalServerError, &error_html);
            response.to_bytes()
        }
    }
}