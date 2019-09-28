" Copy visual selection into system clipboard
xnoremap Y "+y
xnoremap D "+d

" Paste using system clipboard
nnoremap <M-p> "+p|xnoremap <M-p> "+p
nnoremap <M-P> "+P|xnoremap <M-P> "+P

" Copy from cursor to end of line into system clipboard
nnoremap <M-y> "+y$

" Copy from start of line (minus leading whitespace) to end of line into
" system clipboard
nnoremap <M-Y> ^"+y$
