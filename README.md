# sebbocept

intercepts Safe Exam Browser data sent over the seb:// and sebs:// protocols, and instead writes that to a regular .seb file instead of just running it immediately

not sure about if this has any uses, i just made it to get a more of a look under the hood of how it works.

## usage

run it through admin cmd to activate/deactivate intercepting. 

once enabled, all data over seb:// or sebs://, which is sent when websites try to automatically launch seb, is instead written to "intercepted.seb" in your home directory
