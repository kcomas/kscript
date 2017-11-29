

if !exists("main_syntax")
    let main_syntax = 'kscript'
endif

syn match KscriptCommment /[^"']*#[^"']*$/
syn match KscriptVar /[a-z0-9]\+/
syn match KscriptConst /[A-Z0-9]\+/
syn match KscriptString /["][^"]*["]/
syn match KscriptFile /'[^']*'/
syn match KscriptBraces /[\[\]{}()]/
syn match KscriptOperator /[-&<>\=\+\$\?\.]/
syn match KscriptFunction /[\|,]/
syn match KscriptObject /[&@%]/
syn match KscriptIntager /\d\+/
syn match KscriptFloat /\d\+\.\d\+/
syn keyword KsciptBool t f

hi def link KscriptCommment Comment
hi def link KscriptVar Identifier
hi def link KscriptConst Include
hi def link KscriptString String
hi def link KscriptFile SpecialChar
hi def link KscriptBraces Function
hi def link KscriptOperator Operator
hi def link KscriptFunction Delimiter
hi def link KscriptObject StorageClass
hi def link KscriptFloat Float
hi def link KscriptIntager Number
hi def link KsciptBool Boolean
