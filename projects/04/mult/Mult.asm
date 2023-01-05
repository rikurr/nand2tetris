// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.


@R2 // 計算結果の初期化
M=0

@count // 掛け算の計算回数
M=0

(LOOP)

@count
D=M
@R1
D=D-M
@END
D;JEQ // 計算回数が到達したらループを抜ける

@R0 // DにR0の値を格納
D=M

@R2 // R2にM+Dの値を格納
M=D+M

@count // 計算回数をプラス1
M=M+1

@LOOP // ループに戻る
0;JMP


(END)
@END
0;JMP