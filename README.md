## 구현할 거?


[file system]
read_file
  - all source
file_list


[CodeQL]

find_function_refs(filename,functionName)
[ 
  functionCode
  filename
  line
]

find_function_code(filename,functionName)
- function code
- filename
- line number(start)


find_var_definition(filename, line, varname)
- source code
- line number
- file name

find_var_refs(filename, line, varname)
[ 
  source code
  filename
  line
]
