use crate::lang::syntax::parser::shared::block::Block;

use super::scope::Scope;

pub struct BlockAnalyzer;

impl BlockAnalyzer {
    pub fn analyze(block: &Block, parent_scope: &Scope) {}

    pub fn analyze_within_scope(block: &Block, scope: &Scope) {}
}
