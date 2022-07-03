/**
 * This Rust file includes messages those will be digested by consensus reactor
 * Inspired from msgs.go of Tendermint
 * https://github.com/tendermint/tendermint/blob/master/internal/consensus/msgs.go
 * Messages:
 * - Message: generic interface, ValidateBasic()
 * - 
 * - VoteMessage, ProposalMessage, ProposalPOLMessage, NewRoundStepMessage, HasVoteMessage
 * - VoteSetMaj23Message, VoteSetBitsMessage
 */