# Run fortran 77

[onlinegdb.com](https://onlinegdb.com/online_fortran_compiler)

Extra Compiler Flags:

`-std=legacy`

## Errors

1. Масса метилового эфира `Gd` равна `324.3028` (`310.2872`).
   `WAG(4)=322.414`
   `EFIR(4)='Gd'`

2. При выводе `__N_TAG_species_Mole_Parts` перепутаны жирные кислоты в `sn2`:

Порядок: `sn2`, `sn1`, `sn3`:
`395 St Li Li       0.07710`

`395 Li [St->Li] Li`

Это легко проверяется расчетом `Li Li Li` = 0.07710.