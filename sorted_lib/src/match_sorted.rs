use syn::visit_mut::VisitMut;
use syn::{Arm, ExprMatch, Pat, Path};

pub struct MatchCheckReplace {
    pub first_error: Option<syn::Error>,
}

impl VisitMut for MatchCheckReplace {
    fn visit_expr_match_mut(&mut self, node: &mut ExprMatch) {
        if node.attrs.iter().all(|attr| attr.path().is_ident("sorted")) {
            node.attrs.retain(|attr| !attr.path().is_ident("sorted"));

            if self.first_error.is_none() {
                self.check_match_arms(&node.arms);
            }
        }

        syn::visit_mut::visit_expr_mut(self, &mut node.expr);
        for arm in &mut node.arms {
            syn::visit_mut::visit_arm_mut(self, arm);
        }
    }
}

impl MatchCheckReplace {
    pub(crate) fn check_match_arms(&mut self, arms: &[Arm]) {
        let mut sorted_arms = arms.iter().collect::<Vec<_>>();

        // yes, this is inefficient. I should've used Vec<&Arm, String> instead but I'm lazy
        sorted_arms.sort_by_key(|arm| get_string_from_pat(&arm.pat));

        for (sorted_arm, arm) in sorted_arms.iter().zip(arms.iter()) {
            let arm_id = get_string_from_pat(&arm.pat);

            let sorted_arm_id = get_string_from_pat(&sorted_arm.pat);

            if arm_id != sorted_arm_id {
                let msg = format!("{} should sort before {}", sorted_arm_id, arm_id);

                let error = syn::Error::new_spanned(get_path_from_pat(&sorted_arm.pat), msg);

                self.first_error = Some(error);
                return;
            }
        }
    }
}

fn get_string_from_pat(pat: &Pat) -> String {
    match pat {
        Pat::TupleStruct(tuple_struct) => tuple_struct
            .path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()
            .join("::"),
        _ => unimplemented!("get_string_from_pat is only implemented for Pat::TupleStruct"),
    }
}

fn get_path_from_pat(pat: &Pat) -> &Path {
    match pat {
        Pat::TupleStruct(tuple_struct) => &tuple_struct.path,
        _ => unimplemented!("get_path_from_pat is only implemented for Pat::TupleStruct"),
    }
}
