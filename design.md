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

## Needs

- Refactor PCB to be const size.
- Create process list, mapping `pid` to `pcb_addr`
- Indicate whether block in memory is in use.
- Allocate PCB based on next available block.
  - On dealloc, write os.nextAvail to the deallocated block
  - On alloc, os.nextAvail = allocated block's first 2 bytes; then use block
- Impl forking
- Fork exec if cmd ends with `&`
- Impl logging level
  - Log time slice (consider chrono crate)
- `kill` should terminate a running process
- `ps` should print process table

## Wants

- Rename system.rs to os.rs
- Refactor `System.exec()` - extract methods
- Generic I/O
  - Benefits include: built-in scripting, easy redirection
