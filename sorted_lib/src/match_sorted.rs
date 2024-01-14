use std::cmp::Ordering;
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
    fn check_match_arms(&mut self, arms: &[Arm]) {
        let result = self.check_match_arms_inner(arms);

        if let Err(err) = result {
            self.first_error = Some(err);
        }
    }

    fn check_match_arms_inner(&mut self, arms: &[Arm]) -> syn::Result<()> {
        let mut sorted_arms = arms
            .iter()
            .map(|arm| get_string_from_pat(&arm.pat).map(|s| (arm, s)))
            .collect::<syn::Result<Vec<_>>>()?;

        sorted_arms.sort_by(|l, r| l.1.cmp(&r.1));

        for ((sorted_arm, sorted_arm_s), arm) in sorted_arms.iter().zip(arms.iter()) {
            let arm_s = get_string_from_pat(&arm.pat).expect("second iteration failed");

            if &arm_s != sorted_arm_s {
                let msg = format!("{} should sort before {}", sorted_arm_s.0, arm_s.0);

                let error = syn::Error::new_spanned(get_path_from_pat(&sorted_arm.pat), msg);

                return Err(error);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct PatString(pub String);

impl PartialOrd<Self> for PatString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PatString {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0.starts_with('_') {
            Ordering::Greater
        } else if other.0.starts_with('_') {
            Ordering::Less
        } else {
            self.0.cmp(&other.0)
        }
    }
}

fn get_string_from_pat(pat: &Pat) -> syn::Result<PatString> {
    match pat {
        Pat::TupleStruct(tuple_struct) => Ok(PatString(
            tuple_struct
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<_>>()
                .join("::"),
        )),
        Pat::Ident(ident) => Ok(PatString(ident.ident.to_string())),
        Pat::Wild(_) => Ok(PatString("_".to_string())),
        _ => Err(syn::Error::new_spanned(pat, "unsupported by #[sorted]")),
    }
}

fn get_path_from_pat(pat: &Pat) -> &Path {
    match pat {
        Pat::TupleStruct(tuple_struct) => &tuple_struct.path,
        _ => unimplemented!("get_path_from_pat is only implemented for Pat::TupleStruct"),
    }
}
