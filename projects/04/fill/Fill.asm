// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.


@24575 // スクリーンの最大値のアドレス取得
D=A

@limit // スクリーンの最大値のアドレスを変数に
M=D


(LOOP) // 無限ループ

@SCREEN // スクリーン開始アドレスを取得
D=A

@current_screen // 現在のスクリーンアドレスを変数に
M=D

@KBD // キーボードが押されていたら1
D=M

@CREAR // スクリーンをクリア
D;JEQ

(FILL)
@limit // 最大値のアドレスをDレジスタに保存
D=M

@current_screen // 現在のスクリーン位置と最大値を比較
D=M-D

@LOOP // 現在のスクリーン位置が最大値を超えたらENDへ
D;JGT

// スクリーンを黒くする処理
@current_screen // 現在のスクリーンを黒くする
A=M
AM=-1

@current_screen // 現在のスクリーン位置を移動する
M=M+1

@FILL
0;JMP


(CREAR)
@limit // 最大値のアドレスをDレジスタに保存
D=M

@current_screen // 現在のスクリーン位置と最大値を比較
D=M-D

@LOOP // 現在のスクリーン位置が最大値を超えたらENDへ
D;JGT

// スクリーンを白くする処理
@current_screen // 現在のスクリーンを黒くする
A=M
AM=0

@current_screen // 現在のスクリーン位置を移動する
M=M+1

@CREAR
0;JMP