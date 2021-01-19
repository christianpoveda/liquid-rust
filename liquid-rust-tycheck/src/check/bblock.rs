use crate::{
    bblock_env::{BBlockEnv, BBlockTy},
    check::Check,
    glob_env::GlobEnv,
    result::TyResult,
    subtype::Subtype,
    synth::Synth,
};

use liquid_rust_mir::{BBlock, BBlockId};
use liquid_rust_ty::Ty;

impl<'ty, 'env, S: Clone> Check<'ty, 'env, S> for BBlock<S> {
    type Ty = &'ty BBlockTy;
    type Env = (&'env GlobEnv, &'env BBlockEnv, &'env Ty);

    fn check(&self, ty: Self::Ty, env: Self::Env) -> TyResult<S> {
        let mut ty = ty.clone();

        for statement in self.statements() {
            ty.input = statement.synth(&ty.input)?;
        }

        self.terminator().check(&ty, env)
    }
}

impl<'ty, 'env, S: Clone> Check<'ty, 'env, S> for BBlockId {
    type Ty = &'ty BBlockTy;
    type Env = (&'env GlobEnv, &'env BBlockEnv, &'env Ty);

    fn check(&self, ty: Self::Ty, (_, bb_env, _): Self::Env) -> TyResult<S> {
        let bb_ty = bb_env.get_ty(*self);

        bb_ty.subtype(ty, ())
    }
}