var searchIndex = JSON.parse('{\
"lenna_core":{"doc":"","i":[[0,"core","lenna_core","",null,null],[0,"config","lenna_core::core","",null,null],[3,"Config","lenna_core::core::config","",null,null],[12,"pipeline","","",0,null],[3,"ProcessorConfig","","",null,null],[12,"id","","",1,null],[12,"config","","",1,null],[0,"pipeline","lenna_core::core","",null,null],[3,"Pipeline","lenna_core::core::pipeline","",null,null],[11,"new","","",2,[[["config",3],["pool",3]],["pipeline",3]]],[11,"run","","",2,[[["dynamicimage",4]],["dynamicimage",4]]],[11,"process","","",2,[[["dynamicimage",4],["box",3],["option",4],["processorconfig",3]],["dynamicimage",4]]],[0,"pool","lenna_core::core","",null,null],[3,"Pool","lenna_core::core::pool","",null,null],[11,"add","","",3,[[["box",3],["processor",8]]]],[11,"get","","",3,[[],[["box",3],["option",4]]]],[0,"processor","lenna_core::core","",null,null],[8,"Processor","lenna_core::core::processor","",null,null],[11,"id","","",4,[[],["string",3]]],[10,"name","","",4,[[],["string",3]]],[10,"description","","",4,[[],["string",3]]],[10,"process","","",4,[[["processorconfig",3],["dynamicimage",4]],["dynamicimage",4]]],[0,"resize","lenna_core::core","",null,null],[3,"Resize","lenna_core::core::resize","",null,null],[0,"plugins","lenna_core","",null,null],[3,"PluginDeclaration","lenna_core::plugins","",null,null],[12,"rustc_version","","",5,null],[12,"core_version","","",5,null],[12,"register","","",5,null],[8,"PluginRegistrar","","",null,null],[10,"add_plugin","","",6,[[["box",3],["processor",8]]]],[7,"CORE_VERSION","lenna_core","",null,null],[7,"RUSTC_VERSION","","",null,null],[14,"export_plugin","","",null,null],[11,"from","lenna_core::core::config","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"value_into","","",0,[[],["result",4]]],[11,"init","","",0,[[]]],[11,"deref","","",0,[[]]],[11,"deref_mut","","",0,[[]]],[11,"drop","","",0,[[]]],[11,"approx_from","","",0,[[],["result",4]]],[11,"approx_into","","",0,[[],["result",4]]],[11,"value_from","","",0,[[],["result",4]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"vzip","","",0,[[]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"value_into","","",1,[[],["result",4]]],[11,"init","","",1,[[]]],[11,"deref","","",1,[[]]],[11,"deref_mut","","",1,[[]]],[11,"drop","","",1,[[]]],[11,"approx_from","","",1,[[],["result",4]]],[11,"approx_into","","",1,[[],["result",4]]],[11,"value_from","","",1,[[],["result",4]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"vzip","","",1,[[]]],[11,"from","lenna_core::core::pipeline","",2,[[]]],[11,"into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"value_into","","",2,[[],["result",4]]],[11,"init","","",2,[[]]],[11,"deref","","",2,[[]]],[11,"deref_mut","","",2,[[]]],[11,"drop","","",2,[[]]],[11,"approx_from","","",2,[[],["result",4]]],[11,"approx_into","","",2,[[],["result",4]]],[11,"value_from","","",2,[[],["result",4]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"vzip","","",2,[[]]],[11,"from","lenna_core::core::pool","",3,[[]]],[11,"into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"value_into","","",3,[[],["result",4]]],[11,"init","","",3,[[]]],[11,"deref","","",3,[[]]],[11,"deref_mut","","",3,[[]]],[11,"drop","","",3,[[]]],[11,"approx_from","","",3,[[],["result",4]]],[11,"approx_into","","",3,[[],["result",4]]],[11,"value_from","","",3,[[],["result",4]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"vzip","","",3,[[]]],[11,"from","lenna_core::core::resize","",7,[[]]],[11,"into","","",7,[[]]],[11,"to_owned","","",7,[[]]],[11,"clone_into","","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"__clone_box","","",7,[[["private",3]]]],[11,"value_into","","",7,[[],["result",4]]],[11,"init","","",7,[[]]],[11,"deref","","",7,[[]]],[11,"deref_mut","","",7,[[]]],[11,"drop","","",7,[[]]],[11,"approx_from","","",7,[[],["result",4]]],[11,"approx_into","","",7,[[],["result",4]]],[11,"value_from","","",7,[[],["result",4]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"vzip","","",7,[[]]],[11,"from","lenna_core::plugins","",5,[[]]],[11,"into","","",5,[[]]],[11,"to_owned","","",5,[[]]],[11,"clone_into","","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"__clone_box","","",5,[[["private",3]]]],[11,"value_into","","",5,[[],["result",4]]],[11,"init","","",5,[[]]],[11,"deref","","",5,[[]]],[11,"deref_mut","","",5,[[]]],[11,"drop","","",5,[[]]],[11,"approx_from","","",5,[[],["result",4]]],[11,"approx_into","","",5,[[],["result",4]]],[11,"value_from","","",5,[[],["result",4]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"vzip","","",5,[[]]],[11,"name","lenna_core::core::resize","",7,[[],["string",3]]],[11,"description","","",7,[[],["string",3]]],[11,"process","","",7,[[["processorconfig",3],["dynamicimage",4]],["dynamicimage",4]]],[11,"add_plugin","lenna_core::core::pool","",3,[[["box",3],["processor",8]]]],[11,"clone","lenna_core::core::resize","",7,[[],["resize",3]]],[11,"clone","lenna_core::plugins","",5,[[],["plugindeclaration",3]]],[11,"default","lenna_core::core::config","",0,[[]]],[11,"default","lenna_core::core::pipeline","",2,[[]]],[11,"default","lenna_core::core::pool","",3,[[]]],[11,"default","lenna_core::core::resize","",7,[[],["resize",3]]],[11,"fmt","lenna_core::core::config","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"serialize","","",0,[[],["result",4]]],[11,"serialize","","",1,[[],["result",4]]],[11,"deserialize","","",0,[[],["result",4]]],[11,"deserialize","","",1,[[],["result",4]]]],"p":[[3,"Config"],[3,"ProcessorConfig"],[3,"Pipeline"],[3,"Pool"],[8,"Processor"],[3,"PluginDeclaration"],[8,"PluginRegistrar"],[3,"Resize"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);