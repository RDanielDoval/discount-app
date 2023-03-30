use shopify_function::prelude::*;
use shopify_function::Result;
use serde::{Serialize};
// Use the shopify_function crate to generate structs for the function input and output
generate_types!(
    query_path = "./input.graphql",
    schema_path = "./schema.graphql"
);

// Use the shopify_function crate to declare your function entrypoint
#[shopify_function]
fn function(input: input::ResponseData) -> Result<output::FunctionResult> {
    let no_discount = output::FunctionResult {
        discounts: vec![],
        discount_application_strategy: output::DiscountApplicationStrategy::FIRST,
    };

    let cart_lines = input.cart.lines;

    if cart_lines.is_empty() {
        return Ok(no_discount);
    }

    let mut send_discounts = vec![];
    let zero_amount = &String::from("0.0");
    let products = cart_lines.iter()
        .filter_map(|line| match &line.merchandise {
            input::InputCartLinesMerchandise::ProductVariant(variant) => Some(variant),
            input::InputCartLinesMerchandise::CustomProduct => None,
        });
    let mut targets_15 =  vec![];
    let mut targets_17 =  vec![];
    for variant in products {
        if variant.product.has_any_tag {
            let fixed_amount = match &variant.product.metafield {
                Some(meta) => &meta.value,
                None => zero_amount,
            };
            if fixed_amount == "17.0" {
                targets_17.push(output::Target {
                    product_variant: Some(output::ProductVariantTarget {
                        id: variant.id.to_string(),
                        quantity: None,
                    })
                });
            } else {
                targets_15.push(output::Target {
                    product_variant: Some(output::ProductVariantTarget {
                        id: variant.id.to_string(),
                        quantity: None,
                    })
                });
            }
        }
    }

    if targets_15.is_empty() && targets_17.is_empty() {
        return Ok(no_discount);
    }

    if !targets_15.is_empty() {
        send_discounts.push(output::Discount {
            message: Some("ReKeepIt15".to_string()),
            // Apply the discount to the collected targets
            targets: targets_15,
            // Define a percentage-based discount
            value: output::Value {
                fixed_amount: Some(output::FixedAmount {
                    amount: "15.0".to_string(),
                    applies_to_each_item: Some(true)
                }),
                percentage: None
            }
        });
    }    
    
    if !targets_17.is_empty() {
        send_discounts.push(output::Discount {
            message: Some("ReKeepIt17".to_string()),
            // Apply the discount to the collected targets
            targets: targets_17,
            // Define a percentage-based discount
            value: output::Value {
                fixed_amount: Some(output::FixedAmount {
                    amount: "17.0".to_string(),
                    applies_to_each_item: Some(true)
                }),
                percentage: None
            }
        });
    }


    // The shopify_function crate serializes your function result and writes it to STDOUT
    Ok(output::FunctionResult {
        discounts: send_discounts,
        discount_application_strategy: output::DiscountApplicationStrategy::MAXIMUM,
    })
}
#[cfg(test)]
mod tests;