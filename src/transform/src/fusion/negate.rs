// Copyright Materialize, Inc. and contributors. All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.

//! Fuses a sequence of `Negate` operators in to one or zero `Negate` operators.

use mz_expr::MirRelationExpr;

use crate::TransformCtx;

/// Fuses a sequence of `Negate` operators in to one or zero `Negate` operators.
#[derive(Debug)]
pub struct Negate;

impl crate::Transform for Negate {
    fn name(&self) -> &'static str {
        "NegateFusion"
    }

    #[mz_ore::instrument(
        target = "optimizer",
        level = "debug",
        fields(path.segment = "negate_fusion")
    )]
    fn actually_perform_transform(
        &self,
        relation: &mut MirRelationExpr,
        _: &mut TransformCtx,
    ) -> Result<(), crate::TransformError> {
        relation.visit_pre_mut(Self::action);
        mz_repr::explain::trace_plan(&*relation);
        Ok(())
    }
}

impl Negate {
    /// Fuses a sequence of `Negate` operators into one or zero `Negate` operators.
    pub fn action(relation: &mut MirRelationExpr) {
        if let MirRelationExpr::Negate { input } = relation {
            let mut require_negate = true;
            while let MirRelationExpr::Negate { input: inner_input } = &mut **input {
                **input = inner_input.take_dangerous();
                require_negate = !require_negate;
            }

            if !require_negate {
                *relation = input.take_dangerous();
            }
        }
    }
}
