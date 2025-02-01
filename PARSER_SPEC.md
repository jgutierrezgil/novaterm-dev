# Especificación del Parser ANSI

## Gramática Formal (EBNF)
```ebnf
CSI = "\x1b[" [ "?" ] params intermediates* cmd
params = num ( ";" num )*
intermediates = 0x20-0x2F
cmd = 0x40-0x7E

OSC = "\x1b]" args BEL
args = ( any-char - BEL )+
BEL = 0x07
```

## Tabla de Comandos Implementados
| Secuencia          | Tipo       | Parámetros           | Soporte |
|---------------------|------------|----------------------|---------|
| `\x1b[<n>m`         | SGR        | 0-255                | ✓       |
| `\x1b[<row>;<col>H` | CUP        | Enteros positivos    | ✓       |
| `\x1b]8;;<url>\x07` | Hyperlink  | String UTF-8         | ✗       |