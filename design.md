# Design

## RAM layout

RAM is always 10000 (0x2710) bytes.

The process table (PS_TBL) is always 32 (0x0020) bytes (16 processes).

```
        +==RAM_LYT==+
0x0000  | NEXT_BLK  |
        +--PS_TBL--+
0x0002  | PID       |
0x0004  | PCB_ADDR  |
0x0006  | PID       |
0x0008  | PCB_ADDR  |
        | ...       |
0x0020  | PID       |
0x0022  | PCB_ADDR  |
        +-----------+
0x0024  | PCB       |
        +-----------+
0x0424  | PCB       |
        +-----------+
        | ...       |
        +-----------+
0xXXXX  | PCB       |
0x2710  +-----------+
```

## PCB layout

PCB is always 1024 (0x0400) bytes.

```
        +==PCB_LYT==+
0x0000  | PID       |
0x0002  | STATUS    |
0x0004  | DATA_PTR  |
0x0006  | STACK_PTR |
        +--CPU_CTX--+
0x0008  | REG_IP    |
0x000a  | REG_1     |
0x000c  | REG_2     |
0x000e  | REG_3     |
0x0010  | REG_4     |
0x0012  | REG_5     |
0x0014  | REG_6     |
        +-----------+
0x0016  | DATA_BLK  |
        +-----------+
0xXXXX  | STACK_BLK |
0x0400  +-----------+
```

## Program launching steps

1. Allocate PCB.
1. Init header
1. Load instructions
1. Execute

## `ps` output

The "process table" should look like so:

```
pid state exe ip  1 2 3 4 5 6
1   abc   abc x   x x x x x x
2   abc   abc x   x x x x x x
3   abc   abc x   x x x x x x
```

## Forking

1. Shell receives `exec FILE`
1. Shell calls sys.exec(file), attaching it's stdin and stdout unless `&` was
   present
1. System creates an executor and starts it in a different thread
1. Each executor is trying to lock the cpu and pcb mutexes, then executes for a
   time slice

## Wants

- Generic read and write
- Remove compiler mutes
