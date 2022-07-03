// type BaseBlockOperations interface {
// 	Base() uint64
// 	Height() uint64
// 	LoadBlock(height uint64) *types.Block
// 	LoadBlockCommit(height uint64) *types.Commit
// 	LoadSeenCommit(height uint64) *types.Commit
// 	CreateProposalBlock(height uint64, state cstate.LatestBlockState, proposerAddr common.Address, commit *types.Commit) (*types.Block, *types.PartSet)
// 	CommitAndValidateBlockTxs(block *types.Block, lastCommit stypes.LastCommitInfo, byzVals []stypes.Evidence) ([]*types.Validator, common.Hash, error)
// 	CommitBlockTxsIfNotFound(block *types.Block, lastCommit stypes.LastCommitInfo, byzVals []stypes.Evidence) ([]*types.Validator, common.Hash, error)
// 	SaveBlock(block *types.Block, partSet *types.PartSet, seenCommit *types.Commit)
// 	LoadBlockPart(height uint64, index int) *types.Part
// 	LoadBlockMeta(height uint64) *types.BlockMeta
// }
