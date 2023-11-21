#![allow(clippy::all)]
pub use voting::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod voting {
    const _: () = {
        ::core::include_bytes!(
            "/home/inastro/Desktop/University/Masters_2023_sem_1/iden3-voting/backend/abi/Voting.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("treeHeight_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("verifier_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("PERCENTAGE_100"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("PERCENTAGE_100"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("add"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("add"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("element_"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("branches"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("branches"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("commitOnProposal"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("commitOnProposal"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("proposalId_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("commitment_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("commitments"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("commitments"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createProposal"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("createProposal"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("remark_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("proposalData_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct Voting.ProposalData",),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("callData_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("id_"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("distributeFunds"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("distributeFunds"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("recipient_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeProposal"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("executeProposal"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("proposalId_"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getHeight"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getHeight"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLength"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getLength"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRoot"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getRoot"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("leavesCount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("leavesCount"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nullifies"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nullifies"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proposals"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("proposals"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("remark"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct Voting.ProposalParams",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("counters"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct Voting.ProposalCounters",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("callData"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("isExecuted"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proposalsCount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("proposalsCount"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rootsHistory"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("rootsHistory"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verifier"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("verifier"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("voteOnProposal"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("voteOnProposal"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("proposalId_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nullifierHash_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("root_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("proof_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                        2usize,
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(
                                                        256usize
                                                    ),
                                                ),
                                                2usize,
                                            ),
                                        ),
                                        2usize,
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                        2usize,
                                    ),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct VerifierHelper.ProofPoints",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("votingOption_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("enum Voting.VotingOption"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddedCommitment"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AddedCommitment"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("commitment"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProposalCreated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ProposalCreated"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("proposal"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::String,
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                ],),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static VOTING_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct Voting<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Voting<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Voting<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Voting<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Voting<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Voting))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Voting<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                VOTING_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `PERCENTAGE_100` (0xcbf5a55e) function
        pub fn percentage_100(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([203, 245, 165, 94], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `add` (0x446bffba) function
        pub fn add(&self, element: [u8; 32]) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 107, 255, 186], element)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `branches` (0x83cc7660) function
        pub fn branches(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([131, 204, 118, 96], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `commitOnProposal` (0x87088ded) function
        pub fn commit_on_proposal(
            &self,
            proposal_id: ::ethers::core::types::U256,
            commitment: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([135, 8, 141, 237], (proposal_id, commitment))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `commitments` (0x839df945) function
        pub fn commitments(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([131, 157, 249, 69], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createProposal` (0xa81f3227) function
        pub fn create_proposal(
            &self,
            remark: ::std::string::String,
            proposal_data: ProposalData,
            call_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([168, 31, 50, 39], (remark, proposal_data, call_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `distributeFunds` (0x071e9a76) function
        pub fn distribute_funds(
            &self,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 30, 154, 118], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeProposal` (0x0d61b519) function
        pub fn execute_proposal(
            &self,
            proposal_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 97, 181, 25], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getHeight` (0x19efb11d) function
        pub fn get_height(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([25, 239, 177, 29], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLength` (0xbe1c766b) function
        pub fn get_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([190, 28, 118, 107], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoot` (0x5ca1e165) function
        pub fn get_root(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([92, 161, 225, 101], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `leavesCount` (0xb9d13eaf) function
        pub fn leaves_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([185, 209, 62, 175], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nullifies` (0xef6e1f98) function
        pub fn nullifies(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([239, 110, 31, 152], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposals` (0x013cf08b) function
        pub fn proposals(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::std::string::String,
                ProposalParams,
                ProposalCounters,
                ::ethers::core::types::Bytes,
                bool,
            ),
        > {
            self.0
                .method_hash([1, 60, 240, 139], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposalsCount` (0x0a9f46ad) function
        pub fn proposals_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([10, 159, 70, 173], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rootsHistory` (0x241e2cfe) function
        pub fn roots_history(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([36, 30, 44, 254], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifier` (0x2b7ac3f3) function
        pub fn verifier(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([43, 122, 195, 243], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `voteOnProposal` (0xc0bc82e4) function
        pub fn vote_on_proposal(
            &self,
            proposal_id: ::ethers::core::types::U256,
            nullifier_hash: [u8; 32],
            root: [u8; 32],
            proof: ProofPoints,
            voting_option: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [192, 188, 130, 228],
                    (proposal_id, nullifier_hash, root, proof, voting_option),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AddedCommitment` event
        pub fn added_commitment_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AddedCommitmentFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ProposalCreated` event
        pub fn proposal_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ProposalCreatedFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, VotingEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Voting<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "AddedCommitment", abi = "AddedCommitment(uint256,bytes32)")]
    pub struct AddedCommitmentFilter {
        #[ethevent(indexed)]
        pub proposal_id: ::ethers::core::types::U256,
        pub commitment: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "ProposalCreated",
        abi = "ProposalCreated(uint256,(uint256,string,(uint256,uint256,uint256,uint256,uint256),(uint256,uint256,uint256),bytes,bool))"
    )]
    pub struct ProposalCreatedFilter {
        #[ethevent(indexed)]
        pub proposal_id: ::ethers::core::types::U256,
        pub proposal: ProposalInfo,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum VotingEvents {
        AddedCommitmentFilter(AddedCommitmentFilter),
        ProposalCreatedFilter(ProposalCreatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for VotingEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddedCommitmentFilter::decode_log(log) {
                return Ok(VotingEvents::AddedCommitmentFilter(decoded));
            }
            if let Ok(decoded) = ProposalCreatedFilter::decode_log(log) {
                return Ok(VotingEvents::ProposalCreatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for VotingEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddedCommitmentFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalCreatedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddedCommitmentFilter> for VotingEvents {
        fn from(value: AddedCommitmentFilter) -> Self {
            Self::AddedCommitmentFilter(value)
        }
    }
    impl ::core::convert::From<ProposalCreatedFilter> for VotingEvents {
        fn from(value: ProposalCreatedFilter) -> Self {
            Self::ProposalCreatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `PERCENTAGE_100` function with signature `PERCENTAGE_100()` and selector `0xcbf5a55e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "PERCENTAGE_100", abi = "PERCENTAGE_100()")]
    pub struct Percentage100Call;
    ///Container type for all input parameters for the `add` function with signature `add(bytes32)` and selector `0x446bffba`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "add", abi = "add(bytes32)")]
    pub struct AddCall {
        pub element: [u8; 32],
    }
    ///Container type for all input parameters for the `branches` function with signature `branches(uint256)` and selector `0x83cc7660`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "branches", abi = "branches(uint256)")]
    pub struct BranchesCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `commitOnProposal` function with signature `commitOnProposal(uint256,bytes32)` and selector `0x87088ded`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "commitOnProposal", abi = "commitOnProposal(uint256,bytes32)")]
    pub struct CommitOnProposalCall {
        pub proposal_id: ::ethers::core::types::U256,
        pub commitment: [u8; 32],
    }
    ///Container type for all input parameters for the `commitments` function with signature `commitments(bytes32)` and selector `0x839df945`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "commitments", abi = "commitments(bytes32)")]
    pub struct CommitmentsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `createProposal` function with signature `createProposal(string,(uint256,uint256,uint256,uint256,uint256),bytes)` and selector `0xa81f3227`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "createProposal",
        abi = "createProposal(string,(uint256,uint256,uint256,uint256,uint256),bytes)"
    )]
    pub struct CreateProposalCall {
        pub remark: ::std::string::String,
        pub proposal_data: ProposalData,
        pub call_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `distributeFunds` function with signature `distributeFunds(address,uint256)` and selector `0x071e9a76`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "distributeFunds", abi = "distributeFunds(address,uint256)")]
    pub struct DistributeFundsCall {
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `executeProposal` function with signature `executeProposal(uint256)` and selector `0x0d61b519`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "executeProposal", abi = "executeProposal(uint256)")]
    pub struct ExecuteProposalCall {
        pub proposal_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getHeight` function with signature `getHeight()` and selector `0x19efb11d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getHeight", abi = "getHeight()")]
    pub struct GetHeightCall;
    ///Container type for all input parameters for the `getLength` function with signature `getLength()` and selector `0xbe1c766b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getLength", abi = "getLength()")]
    pub struct GetLengthCall;
    ///Container type for all input parameters for the `getRoot` function with signature `getRoot()` and selector `0x5ca1e165`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getRoot", abi = "getRoot()")]
    pub struct GetRootCall;
    ///Container type for all input parameters for the `leavesCount` function with signature `leavesCount()` and selector `0xb9d13eaf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "leavesCount", abi = "leavesCount()")]
    pub struct LeavesCountCall;
    ///Container type for all input parameters for the `nullifies` function with signature `nullifies(bytes32)` and selector `0xef6e1f98`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "nullifies", abi = "nullifies(bytes32)")]
    pub struct NullifiesCall(pub [u8; 32]);
    ///Container type for all input parameters for the `proposals` function with signature `proposals(uint256)` and selector `0x013cf08b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "proposals", abi = "proposals(uint256)")]
    pub struct ProposalsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `proposalsCount` function with signature `proposalsCount()` and selector `0x0a9f46ad`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "proposalsCount", abi = "proposalsCount()")]
    pub struct ProposalsCountCall;
    ///Container type for all input parameters for the `rootsHistory` function with signature `rootsHistory(bytes32)` and selector `0x241e2cfe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "rootsHistory", abi = "rootsHistory(bytes32)")]
    pub struct RootsHistoryCall(pub [u8; 32]);
    ///Container type for all input parameters for the `verifier` function with signature `verifier()` and selector `0x2b7ac3f3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "verifier", abi = "verifier()")]
    pub struct VerifierCall;
    ///Container type for all input parameters for the `voteOnProposal` function with signature `voteOnProposal(uint256,bytes32,bytes32,(uint256[2],uint256[2][2],uint256[2]),uint8)` and selector `0xc0bc82e4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "voteOnProposal",
        abi = "voteOnProposal(uint256,bytes32,bytes32,(uint256[2],uint256[2][2],uint256[2]),uint8)"
    )]
    pub struct VoteOnProposalCall {
        pub proposal_id: ::ethers::core::types::U256,
        pub nullifier_hash: [u8; 32],
        pub root: [u8; 32],
        pub proof: ProofPoints,
        pub voting_option: u8,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum VotingCalls {
        Percentage100(Percentage100Call),
        Add(AddCall),
        Branches(BranchesCall),
        CommitOnProposal(CommitOnProposalCall),
        Commitments(CommitmentsCall),
        CreateProposal(CreateProposalCall),
        DistributeFunds(DistributeFundsCall),
        ExecuteProposal(ExecuteProposalCall),
        GetHeight(GetHeightCall),
        GetLength(GetLengthCall),
        GetRoot(GetRootCall),
        LeavesCount(LeavesCountCall),
        Nullifies(NullifiesCall),
        Proposals(ProposalsCall),
        ProposalsCount(ProposalsCountCall),
        RootsHistory(RootsHistoryCall),
        Verifier(VerifierCall),
        VoteOnProposal(VoteOnProposalCall),
    }
    impl ::ethers::core::abi::AbiDecode for VotingCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <Percentage100Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Percentage100(decoded));
            }
            if let Ok(decoded) = <AddCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Add(decoded));
            }
            if let Ok(decoded) = <BranchesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Branches(decoded));
            }
            if let Ok(decoded) =
                <CommitOnProposalCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CommitOnProposal(decoded));
            }
            if let Ok(decoded) = <CommitmentsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Commitments(decoded));
            }
            if let Ok(decoded) =
                <CreateProposalCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CreateProposal(decoded));
            }
            if let Ok(decoded) =
                <DistributeFundsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DistributeFunds(decoded));
            }
            if let Ok(decoded) =
                <ExecuteProposalCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExecuteProposal(decoded));
            }
            if let Ok(decoded) = <GetHeightCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetHeight(decoded));
            }
            if let Ok(decoded) = <GetLengthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetLength(decoded));
            }
            if let Ok(decoded) = <GetRootCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRoot(decoded));
            }
            if let Ok(decoded) = <LeavesCountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LeavesCount(decoded));
            }
            if let Ok(decoded) = <NullifiesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nullifies(decoded));
            }
            if let Ok(decoded) = <ProposalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Proposals(decoded));
            }
            if let Ok(decoded) =
                <ProposalsCountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProposalsCount(decoded));
            }
            if let Ok(decoded) = <RootsHistoryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RootsHistory(decoded));
            }
            if let Ok(decoded) = <VerifierCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Verifier(decoded));
            }
            if let Ok(decoded) =
                <VoteOnProposalCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VoteOnProposal(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for VotingCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Percentage100(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Add(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Branches(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CommitOnProposal(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Commitments(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CreateProposal(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DistributeFunds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteProposal(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetHeight(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetLength(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRoot(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LeavesCount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nullifies(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Proposals(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProposalsCount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RootsHistory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Verifier(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VoteOnProposal(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for VotingCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Percentage100(element) => ::core::fmt::Display::fmt(element, f),
                Self::Add(element) => ::core::fmt::Display::fmt(element, f),
                Self::Branches(element) => ::core::fmt::Display::fmt(element, f),
                Self::CommitOnProposal(element) => ::core::fmt::Display::fmt(element, f),
                Self::Commitments(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateProposal(element) => ::core::fmt::Display::fmt(element, f),
                Self::DistributeFunds(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteProposal(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetHeight(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoot(element) => ::core::fmt::Display::fmt(element, f),
                Self::LeavesCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nullifies(element) => ::core::fmt::Display::fmt(element, f),
                Self::Proposals(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalsCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::RootsHistory(element) => ::core::fmt::Display::fmt(element, f),
                Self::Verifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::VoteOnProposal(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<Percentage100Call> for VotingCalls {
        fn from(value: Percentage100Call) -> Self {
            Self::Percentage100(value)
        }
    }
    impl ::core::convert::From<AddCall> for VotingCalls {
        fn from(value: AddCall) -> Self {
            Self::Add(value)
        }
    }
    impl ::core::convert::From<BranchesCall> for VotingCalls {
        fn from(value: BranchesCall) -> Self {
            Self::Branches(value)
        }
    }
    impl ::core::convert::From<CommitOnProposalCall> for VotingCalls {
        fn from(value: CommitOnProposalCall) -> Self {
            Self::CommitOnProposal(value)
        }
    }
    impl ::core::convert::From<CommitmentsCall> for VotingCalls {
        fn from(value: CommitmentsCall) -> Self {
            Self::Commitments(value)
        }
    }
    impl ::core::convert::From<CreateProposalCall> for VotingCalls {
        fn from(value: CreateProposalCall) -> Self {
            Self::CreateProposal(value)
        }
    }
    impl ::core::convert::From<DistributeFundsCall> for VotingCalls {
        fn from(value: DistributeFundsCall) -> Self {
            Self::DistributeFunds(value)
        }
    }
    impl ::core::convert::From<ExecuteProposalCall> for VotingCalls {
        fn from(value: ExecuteProposalCall) -> Self {
            Self::ExecuteProposal(value)
        }
    }
    impl ::core::convert::From<GetHeightCall> for VotingCalls {
        fn from(value: GetHeightCall) -> Self {
            Self::GetHeight(value)
        }
    }
    impl ::core::convert::From<GetLengthCall> for VotingCalls {
        fn from(value: GetLengthCall) -> Self {
            Self::GetLength(value)
        }
    }
    impl ::core::convert::From<GetRootCall> for VotingCalls {
        fn from(value: GetRootCall) -> Self {
            Self::GetRoot(value)
        }
    }
    impl ::core::convert::From<LeavesCountCall> for VotingCalls {
        fn from(value: LeavesCountCall) -> Self {
            Self::LeavesCount(value)
        }
    }
    impl ::core::convert::From<NullifiesCall> for VotingCalls {
        fn from(value: NullifiesCall) -> Self {
            Self::Nullifies(value)
        }
    }
    impl ::core::convert::From<ProposalsCall> for VotingCalls {
        fn from(value: ProposalsCall) -> Self {
            Self::Proposals(value)
        }
    }
    impl ::core::convert::From<ProposalsCountCall> for VotingCalls {
        fn from(value: ProposalsCountCall) -> Self {
            Self::ProposalsCount(value)
        }
    }
    impl ::core::convert::From<RootsHistoryCall> for VotingCalls {
        fn from(value: RootsHistoryCall) -> Self {
            Self::RootsHistory(value)
        }
    }
    impl ::core::convert::From<VerifierCall> for VotingCalls {
        fn from(value: VerifierCall) -> Self {
            Self::Verifier(value)
        }
    }
    impl ::core::convert::From<VoteOnProposalCall> for VotingCalls {
        fn from(value: VoteOnProposalCall) -> Self {
            Self::VoteOnProposal(value)
        }
    }
    ///Container type for all return fields from the `PERCENTAGE_100` function with signature `PERCENTAGE_100()` and selector `0xcbf5a55e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Percentage100Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `branches` function with signature `branches(uint256)` and selector `0x83cc7660`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BranchesReturn(pub [u8; 32]);
    ///Container type for all return fields from the `commitments` function with signature `commitments(bytes32)` and selector `0x839df945`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CommitmentsReturn(pub bool);
    ///Container type for all return fields from the `createProposal` function with signature `createProposal(string,(uint256,uint256,uint256,uint256,uint256),bytes)` and selector `0xa81f3227`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CreateProposalReturn {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getHeight` function with signature `getHeight()` and selector `0x19efb11d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetHeightReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getLength` function with signature `getLength()` and selector `0xbe1c766b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getRoot` function with signature `getRoot()` and selector `0x5ca1e165`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetRootReturn(pub [u8; 32]);
    ///Container type for all return fields from the `leavesCount` function with signature `leavesCount()` and selector `0xb9d13eaf`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LeavesCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `nullifies` function with signature `nullifies(bytes32)` and selector `0xef6e1f98`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NullifiesReturn(pub bool);
    ///Container type for all return fields from the `proposals` function with signature `proposals(uint256)` and selector `0x013cf08b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ProposalsReturn {
        pub id: ::ethers::core::types::U256,
        pub remark: ::std::string::String,
        pub params: ProposalParams,
        pub counters: ProposalCounters,
        pub call_data: ::ethers::core::types::Bytes,
        pub is_executed: bool,
    }
    ///Container type for all return fields from the `proposalsCount` function with signature `proposalsCount()` and selector `0x0a9f46ad`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ProposalsCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rootsHistory` function with signature `rootsHistory(bytes32)` and selector `0x241e2cfe`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RootsHistoryReturn(pub bool);
    ///Container type for all return fields from the `verifier` function with signature `verifier()` and selector `0x2b7ac3f3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct VerifierReturn(pub ::ethers::core::types::Address);
    ///`ProofPoints(uint256[2],uint256[2][2],uint256[2])`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ProofPoints {
        pub a: [::ethers::core::types::U256; 2],
        pub b: [[::ethers::core::types::U256; 2]; 2],
        pub c: [::ethers::core::types::U256; 2],
    }
    ///`ProposalCounters(uint256,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ProposalCounters {
        pub votes_for: ::ethers::core::types::U256,
        pub votes_against: ::ethers::core::types::U256,
        pub commitments: ::ethers::core::types::U256,
    }
    ///`ProposalData(uint256,uint256,uint256,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ProposalData {
        pub commitment_period: ::ethers::core::types::U256,
        pub voting_period: ::ethers::core::types::U256,
        pub proposal_execution_period: ::ethers::core::types::U256,
        pub required_quorum: ::ethers::core::types::U256,
        pub required_majority: ::ethers::core::types::U256,
    }
    ///`ProposalInfo(uint256,string,(uint256,uint256,uint256,uint256,uint256),(uint256,uint256,uint256),bytes,bool)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ProposalInfo {
        pub id: ::ethers::core::types::U256,
        pub remark: ::std::string::String,
        pub params: ProposalParams,
        pub counters: ProposalCounters,
        pub call_data: ::ethers::core::types::Bytes,
        pub is_executed: bool,
    }
    ///`ProposalParams(uint256,uint256,uint256,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ProposalParams {
        pub voting_end_time: ::ethers::core::types::U256,
        pub commitment_end_time: ::ethers::core::types::U256,
        pub proposal_execution_end_time: ::ethers::core::types::U256,
        pub required_quorum: ::ethers::core::types::U256,
        pub required_majority: ::ethers::core::types::U256,
    }
}
