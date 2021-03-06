use liquid_rust_mir::ty::{BaseTy, Predicate};

use crate::{
    ast::Ident,
    resolution::{
        solve::{ResolutionErrorKind, ResolutionResult, Solve},
        ResolutionCtx,
    },
};

impl<'source> Solve<'source> for Ident<'source> {
    type Output = Predicate;

    fn solve(
        &self,
        rcx: &mut ResolutionCtx<'source>,
    ) -> ResolutionResult<'source, (Self::Output, BaseTy)> {
        // Traverse the stack of scopes from top to bottom.
        for (index, scope) in rcx.scopes() {
            // If the identifier is in the current scope, return it.
            if let Some((mut predicate, base_ty)) = scope.solve_ident(self) {
                // If the variable bound to the identifier is an argument. Increase its de Bruijn
                // index using the index of the current scope.
                if let Predicate::Arg(arg) = &mut predicate {
                    *arg = arg.inc(index);
                }

                return Ok((predicate, base_ty));
            }
        }
        // The identifier is not bound. Return an error.
        ResolutionErrorKind::UnboundIdent(self.symbol).into_err(self.span.clone())
    }
}
