(function() {var implementors = {};
implementors["basic_contract_rs"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"fabric_contract/ledgerapi/state/struct.State.html\" title=\"struct fabric_contract::ledgerapi::state::State\">State</a>&gt; for <a class=\"struct\" href=\"basic_contract_rs/struct.MyAsset.html\" title=\"struct basic_contract_rs::MyAsset\">MyAsset</a>","synthetic":false,"types":["basic_contract_rs::types::myasset::MyAsset"]}];
implementors["fabric_contract"] = [{"text":"impl&lt;'_&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'_ <a class=\"struct\" href=\"fabric_contract/data/struct.WireBuffer.html\" title=\"struct fabric_contract::data::WireBuffer\">WireBuffer</a>&gt; for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>","synthetic":false,"types":["alloc::string::String"]},{"text":"impl&lt;'_&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'_ <a class=\"struct\" href=\"fabric_contract/data/struct.WireBuffer.html\" title=\"struct fabric_contract::data::WireBuffer\">WireBuffer</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"struct\" href=\"fabric_contract/contract/struct.ContractError.html\" title=\"struct fabric_contract::contract::ContractError\">ContractError</a>","synthetic":false,"types":["fabric_contract::error::ContractError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"struct\" href=\"fabric_contract/contract/struct.LedgerError.html\" title=\"struct fabric_contract::contract::LedgerError\">LedgerError</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>&gt; for <a class=\"struct\" href=\"fabric_contract/contract/struct.ContractError.html\" title=\"struct fabric_contract::contract::ContractError\">ContractError</a>","synthetic":false,"types":["fabric_contract::error::ContractError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"fabric_contract/contract/struct.LedgerError.html\" title=\"struct fabric_contract::contract::LedgerError\">LedgerError</a>&gt; for <a class=\"struct\" href=\"fabric_contract/contract/struct.ContractError.html\" title=\"struct fabric_contract::contract::ContractError\">ContractError</a>","synthetic":false,"types":["fabric_contract::error::ContractError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"struct\" href=\"fabric_contract/contract/struct.LedgerError.html\" title=\"struct fabric_contract::contract::LedgerError\">LedgerError</a>","synthetic":false,"types":["fabric_contract::error::LedgerError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>&gt; for <a class=\"struct\" href=\"fabric_contract/contract/struct.State.html\" title=\"struct fabric_contract::contract::State\">State</a>","synthetic":false,"types":["fabric_contract::ledgerapi::state::State"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>&gt; for <a class=\"struct\" href=\"fabric_contract/contract/struct.State.html\" title=\"struct fabric_contract::contract::State\">State</a>","synthetic":false,"types":["fabric_contract::ledgerapi::state::State"]},{"text":"impl&lt;'_&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'_ State&gt; for <a class=\"struct\" href=\"fabric_contract/contract/struct.State.html\" title=\"struct fabric_contract::contract::State\">State</a>","synthetic":false,"types":["fabric_contract::ledgerapi::state::State"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;State&gt; for <a class=\"struct\" href=\"fabric_contract/contract/struct.State.html\" title=\"struct fabric_contract::contract::State\">State</a>","synthetic":false,"types":["fabric_contract::ledgerapi::state::State"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"fabric_contract/contract/struct.State.html\" title=\"struct fabric_contract::contract::State\">State</a>&gt; for State","synthetic":false,"types":["fabric_ledger_protos::ledger_messages::State"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()