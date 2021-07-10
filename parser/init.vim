
if &compatible
    set nocompatible               " Be iMproved
endif

let mapleader="," 
" Required: 
filetype plugin indent on
" Only set syntax highlighting once!
if !exists("g:syntax_on")
    syntax enable
endif 

call plug#begin('~/.config/nvim/plugged')
Plug 'alvan/vim-closetag'
Plug 'jiangmiao/auto-pairs'
Plug 'tpope/vim-surround'
Plug 'wellle/targets.vim'
Plug 'tpope/vim-repeat'
Plug 'airblade/vim-gitgutter'
Plug 'vim-airline/vim-airline'
Plug 'honza/vim-snippets'
Plug 'scrooloose/nerdcommenter'
Plug 'arcticicestudio/nord-vim'
Plug 'tommcdo/vim-exchange'
Plug '/usr/local/opt/fzf'
Plug 'junegunn/fzf.vim'
Plug 'prettier/vim-prettier', { 'do': 'yarn install' }
Plug 'ryanoasis/vim-devicons'
Plug 'machakann/vim-highlightedyank'
Plug 'sainnhe/sonokai'
Plug 'hrsh7th/nvim-compe'
Plug 'neovim/nvim-lspconfig'
Plug 'nvim-lua/popup.nvim'
Plug 'nvim-lua/plenary.nvim'
Plug 'nvim-telescope/telescope.nvim'
Plug 'nvim-treesitter/nvim-treesitter', {'do': ':TSUpdate'}  " We recommend updating the parsers on update


call plug#end()

function! MakeSession()
    let b:sessiondir = $HOME . "/.config/nvim/sessions" . getcwd()
    if (filewritable(b:sessiondir) != 2)
        exe 'silent !mkdir -p ' b:sessiondir
        redraw!
    endif
   let b:filename = b:sessiondir . '/session.vim'
    exe "mksession! " . b:filename
endfunction

function! LoadSession()
    let b:sessiondir = $HOME . "/.config/nvim/sessions" . getcwd()
    let b:sessionfile = b:sessiondir . "/session.vim"
    if (filereadable(b:sessionfile))
        exe 'source ' b:sessionfile
    else
        echo "No session loaded."
    endif
endfunction

" Controls the open/close Vim functions
augroup vimSessions
    autocmd!
    " Adding automatons for when entering or leaving Vim
    au VimEnter * nested :call LoadSession()
   au VimLeave * :call MakeSession()
augroup END

augroup markdownSpell
    autocmd!
    autocmd FileType markdown setlocal spell
    autocmd BufNewFile,BufRead *.txt set syntax=markdown
    autocmd FileType text setlocal spell
    autocmd BufRead,BufNewFile *.md,*.txt setlocal spell
    " Stop any default wrapping when in txt or markdown
    set textwidth=0
    " When editing a file, always jump to the last known cursor position.
    " Don't do it for commit messages, when the position is invalid, or when
    " inside an event handler (happens when dropping a file on gvim).
    autocmd BufReadPost *
                \ if &ft != 'gitcommit' && line("'\"") > 0 && line("'\"") <= line("$") |
                \   exe "normal g`\"" |
                \ endif
augroup END



" Prevent CTRL+Z suspending Vim
nnoremap <c-z> <nop

" Files + devicons
function! Fzf_dev()
  let l:fzf_files_options = '--preview "bat --theme="OneHalfDark" --style=numbers,changes --color always {2..-1} | head -'.&lines.'"'

  function! s:files()
    let l:files = split(system($FZF_DEFAULT_COMMAND), '\n')
    return s:prepend_icon(l:files)
  endfunction


  function! s:prepend_icon(candidates)
    let l:result = []
    for l:candidate in a:candidates
      let l:filename = fnamemodify(l:candidate, ':p:t')
      let l:icon = WebDevIconsGetFileTypeSymbol(l:filename, isdirectory(l:filename))
      call add(l:result, printf('%s %s', l:icon, l:candidate))
    endfor

    return l:result
  endfunction

  function! s:edit_file(item)
    let l:pos = stridx(a:item, ' ')
    let l:file_path = a:item[pos+1:-1]
    execute 'silent e' l:file_path
  endfunction

  call fzf#run({
        \ 'source': <sid>files(),
        \ 'sink':   function('s:edit_file'),
        \ 'options': '-m ' . l:fzf_files_options,
        \ 'down':    '40%' })
endfunction


if !&scrolloff
    set scrolloff=3       " Show next 3 lines while scrolling.
endif
if !&sidescrolloff
    set sidescrolloff=5   " Show next 5 columns while side-scrolling.
endif

augroup numbertoggle
    autocmd!
    autocmd BufEnter,FocusGained,InsertLeave * set relativenumber
    autocmd BufLeave,FocusLost,InsertEnter   * set norelativenumber
augroup END

" Prettier - to enable format on save
let g:prettier#autoformat = 0
augroup prettier
    autocmd!
    autocmd BufWritePre *.js,*.jsx,*.mjs,*.ts,*.tsx,*.css,*.less,*.scss,*.json,*.graphql,*.md,*.vue,*.yaml,*.html PrettierAsync
augroup END

" Airline configuration
":let g:airline_extensions = []
let g:airline_theme='nord'
let g:airline#extensions#coc#enabled = 1
"let g:airline_symbols = {}
"let g:airline_symbols.maxlinenr = ''
"let g:airline_symbols.linenr = ''
let g:airline_right_sep = ''
let g:airline_right_alt_sep = ''
" 
"call airline#parts#define_accent('mode', 'bold')
"let g:airline_section_a = airline#section#create(['mode', ' '])


let g:airline#parts#ffenc#skip_expected_string='utf-8[unix]'
"let g:airline_section_y=0
" Just show the filename (no path) in the tab
let g:airline#extensions#tabline#fnamemod = ':t'
let g:airline_powerline_fonts = 1
" Don't bother telling me about whitespace
silent! call airline#extensions#whitespace#disable()
" Don't bother telling me about spelling
let g:airline_detect_spell=0
" Show system time
let g:airline_section_b = '%{strftime("%H:%M")}'
" Coc Vim stuff
" Better display for messages - this makes the command line area bigger.
" Default is set cmdheight=1
"set cmdheight=2

" You will have bad experience for diagnostic messages when it's default 4000.
set updatetime=300

" don't give |ins-completion-menu| messages.
set shortmess+=c

" always show signcolumns
set signcolumn=yes

" Make splits open in more natuural locations
set splitbelow
set splitright

" Now make navigating between splits a little easier. Just use leader h,j,k,l

nnoremap <Leader>j <C-W><C-J>
nnoremap <Leader>k <C-W><C-K>
nnoremap <Leader>l <C-W><C-L>
nnoremap <Leader>h <C-W><C-H>


let g:compe = {}
let g:compe.enabled = v:true
let g:compe.autocomplete = v:true
let g:compe.debug = v:false
let g:compe.min_length = 1
let g:compe.preselect = 'enable'
let g:compe.throttle_time = 80
let g:compe.source_timeout = 200
let g:compe.incomplete_delay = 400
let g:compe.max_abbr_width = 100
let g:compe.max_kind_width = 100
let g:compe.max_menu_width = 100
let g:compe.documentation = v:true

let g:compe.source = {}
let g:compe.source.path = v:true
let g:compe.source.buffer = v:true
let g:compe.source.calc = v:true
let g:compe.source.nvim_lsp = v:true
let g:compe.source.nvim_lua = v:true
let g:compe.source.vsnip = v:true
let g:compe.source.ultisnips = v:true

inoremap <silent><expr> <C-Space> compe#complete()
inoremap <silent><expr> <CR>      compe#confirm('<CR>')
inoremap <silent><expr> <C-e>     compe#close('<C-e>')
inoremap <silent><expr> <C-f>     compe#scroll({ 'delta': +4 })
inoremap <silent><expr> <C-d>     compe#scroll({ 'delta': -4 })


"" After searching, pressing escape stops the highlight
nnoremap <silent> <esc> :noh<cr><esc>

" Open nvimrc file
nnoremap <Leader>v :vsp $MYVIMRC<CR>

" Source nvimrc file
nnoremap <Leader>sv :source $MYVIMRC<CR>

" Quick new file
nnoremap <Leader>n :enew<CR>

" Easy open devDiary file
nnoremap <Leader>od :edit ~/Desktop/DevDiary/diary.md<CR>

" Easy select all of file
nnoremap <Leader>sa ggvG<c-$>

" Easy show registers
nnoremap <silent> <Leader>zz :reg<CR>


" Make visual yanks pace the cursor back where started
" adding lazyredraw as it should help with the flicker
"set lazyredraw
vmap y ygv<Esc>

" Line bubbling
" Use these two if you don't have prettier
"nnoremap <silent> <c-j> :m .+1<CR>==
"nnoremap <silent> <c-k> :m .-2<CR>==
nnoremap <silent> <c-j> :m .+1<CR>
nnoremap <silent> <c-k> :m .-2<CR>
inoremap <silent> <c-j> <Esc>:m .+1<CR>==gi
inoremap <silent> <c-k> <Esc>:m .-2<CR>==gi
vnoremap <silent> <c-j> :m '>+1<CR>gv=gv
vnoremap <silent> <c-k> :m '<-2<CR>gv=gv

"Auto close tags
imap ,/ </<C-X><C-O>

if exists('+termguicolors') && ($TERM == "st-256color" || $TERM == "tmux-256color")
    let &t_8f = "\<Esc>[38;2;%lu;%lu;%lum"
    let &t_8b = "\<Esc>[48;2;%lu;%lu;%lum"
    set termguicolors
endif

set guicursor=n-v-c:block,i-ci-ve:ver25,r-cr:hor20,o:hor50,a:blinkwait700-blinkoff400-blinkon250-Cursor/lCursor,sm:block-blinkwait175-blinkoff150-blinkon175

" Nord theme config
" let g:nord_italic = 1
" let g:nord_italic_comments = 1
" let g:nord_underline = 1
" let g:nord_uniform_status_lines = 1
" let g:nord_uniform_diff_background = 1
" let g:nord_cursor_line_number_background = 1
" 
" colorscheme nord

" The configuration options should be placed before `colorscheme sonokai`.
let g:sonokai_style = 'andromeda'
let g:sonokai_enable_italic = 1
let g:sonokai_disable_italic_comment = 1
colorscheme sonokai

" Telescope
nnoremap <leader>ff <cmd>lua require('telescope.builtin').find_files(require('telescope.themes').get_dropdown({}))<cr>
nnoremap <leader>fg <cmd>lua require('telescope.builtin').live_grep()<cr>
nnoremap <leader>fb <cmd>lua require('telescope.builtin').buffers()<cr>
nnoremap <leader>fh <cmd>lua require('telescope.builtin').help_tags()<cr>



" Set matching bracket for 1 second
set mat=1
" Cursor line but only current window and not when inserting text
set cursorline
autocmd InsertLeave,WinEnter * set cursorline
autocmd InsertEnter,WinLeave * set nocursorline

" Get a preview of replacements
set inccommand=split

" Use spelling for markdown files ‘]s’ to find next, ‘[s’ for previous, 'z=‘ for suggestions when on one.
" Source: http://thejakeharding.com/tutorial/2012/06/13/using-spell-check-in-vim.html
set spelllang=en
set spellsuggest?
set wrap linebreak nolist
set formatoptions=l
set hidden

" Make the gutter for numbers wider by default
set numberwidth=5

set display+=lastline
set clipboard=unnamedplus
set mouse=
set ruler               " Show the line and column numbers of the cursor.
set autoread
set noshowmode                    " Show current mode.
set encoding=utf-8              " Set default encoding to UTF-8
set incsearch                   " Shows the match while typing
set hlsearch                    " Highlight found searches
set ignorecase                  " Search case insensitive...
set smartcase                   " ... but not when search pattern contains upper case characters
set autoindent
set tabstop=4 shiftwidth=4 expandtab
"set gdefault " use g flag by default on searches
set number relativenumber

" Autocomplete with dictionary words when spell check is on
set complete+=kspell

set completeopt=menuone,noselect
