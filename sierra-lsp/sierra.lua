local util = require 'lspconfig.util'

local bin_name  = 'sierra-lsp'
local cmd = {bin_name}

return {
  default_config = {
    cmd = cmd,
    filetypes = {'sierra'},
    root_dir = util.root_pattern('package.sierra', '.git'),
    single_file_support = true,
    settings = {
      sierra = { validate = true},
    },
  },
  docs = {
    description = [[
      Sierra language server
    ]],
    default_config = {
      root_dir = [[root_pattern("package.sierra", ".git") or bufdir ]]
    }
  }
}
