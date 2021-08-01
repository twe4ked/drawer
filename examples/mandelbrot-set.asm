; https://gist.github.com/haileysome/aacf43c349e19497c24b2edb198c5702

    STO H 1

res:
    STO X 0
    SUB X 512

    STO Y 0
    SUB Y 512

pix:
    # save X and Y
    STO V X
    STO W Y

    # abuse MOV to get 1/2
    STO X 0
    STO Y 0
    STO A 60
    MOV
    STO S X

    # reset X and Y
    STO X V
    STO Y W

    # raise S to the 10th power to get 1/1024
    STO T 1
    MUL T S
    MUL T S
    MUL T S
    MUL T S
    MUL T S
    MUL T S
    MUL T S
    MUL T S
    MUL T S
    MUL T S

    # V <- V * 3 / 1024 - 0.5
    MUL V 4
    MUL V T
    SUB V S

    # W <- W * 4 / 1024
    MUL W 4
    MUL W T

    STO S 0
    STO T 0
    STO B 0

# LOOP INVARIANT
# z = S + T*i
# c = V + W*i
# z^2 = S^2 - T^2 + 2*S*T*i
iter:
    STO U S
    STO Z T

    MUL U U
    MUL Z Z
    SUB U Z
    # U = z^2 real

    STO Z 2
    MUL Z S
    MUL Z T
    # Z = y^2 imag

    ADD U V
    ADD Z W
    # U+Z*i = z^2 + c

    STO S U
    STO T Z
    # z <- z^2 + c
    # loop invariant now holds

    # calculate abs(z) and see if we've escaped
    MUL U U
    MUL Z Z
    ADD U Z
    JGT U 4 escape:

    INC B
    JGT B H limit:

    # unconditional jump
    JNZ B iter:

escape:
    DRW
    DRW

limit:

next:
    INC X
    JLT X 512 pix:

    STO X 0
    SUB X 512
    INC Y
    JLT Y 512 pix:

    INC H
    JLT H 15 res:

    HLT
