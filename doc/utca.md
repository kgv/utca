# UTCA

Программа условно разделена на 5 составляющих: `Configuration`, `Calculation`,
`Composition`, `Visualization` и `Comparison`.

## Configuration

### Settings

- `C` - задает минимальное и максимальное значение для `C`.
- `U` - задает максимальное значение для `U`.

## Calculation

Введенные конфигурационные данные нормализуются.

Для расчета значений столбцов `1,2/2,3-DAG` и `2-MAG` можно выбрать один из двух
источников данных: `Experimental` или `Calculation`. По умолчанию -
`Experimental`.

- `Experimental` - значения получаются нормализацией данных соответствующего
  столбца.

- `Calculation` - значения расчитываются по соответствующим взаимобратным
  формулам:

  - `1,2,3-TAG` = (4 * `1,2/2,3-DAG` - `2-MAG`) / 3
  - `1,2/2,3-DAG` = (3 * `1,2,3-TAG` + `2-MAG`) / 4
  - `2-MAG` = 4 * `1,2/2,3-DAG` - 3 * `1,2,3-TAG`[^1][^pin341.for/56]

Для расчета значений столбца `1,3-DAG` также можно выбрать один из двух
источников данных: `1,2/2,3-DAG` или `2-MAG`. По умолчанию - `1,2/2,3-DAG`.

Значения расчитываются по соответствующим формулам:

- `1,3-DAG` = 3 * `1,2,3-TAG` - 2 * `1,2/2,3-DAG`[^pin341.for/68]
- `1,3-DAG` = (3 * `1,2,3-TAG` - `2-MAG`) / 2[^1][^4]

### Settings

Параметр `Normalization` может быть установлен в один из трех вариантов: `Mass`,
`Molar` или `Pchelkin`.

- `Mass` - нормализация данных не учитывает поправку на молярную массу ионов.
  Нормализация осуществляется по формуле: `s/∑(s)`, где `s` - плошадь пика
  соответствующего иона.
- `Molar` - нормализация данных учитывает поправку на молярную массу ионов.
  Нормализация осуществляется по формуле: `(s * m)/∑(s * m)`, где `s` - плошадь
  пика, а `m` - масса соответствующего иона.
- `Pchelkin` - соотношение данных учитывает поправку на молярную массу ионов.
  Отношение осуществляется по формуле: `s / ∑(s * m / 10)`, где `s` - плошадь
  пика, а `m` - масса соответствующего иона. Это отношение не является
  нормализующим.

Параметр `Signedness` может быть установлен в один из двух вариантов: `Signed`,
`Unsigned`. По умолчанию - `Signed`.

При расчете `2-MAG` с установленным параметром `Calculation` могут получится
отриательные значения, что корректно с математической точки зрения, но не
корректно с физической точки зрения. В таких случаях требуется установка
параметра в значение `Unsigned`, чтобы отрицательные значения преобразовавались
в нулевые.

Программа Пчелкина:

- `1,2/2,3-DAG`: `Experimental`
- `2-MAG`: `Calculation`
- `1,3-DAG`: `1,2/2,3-DAG`
- `Normalization`: `Pchelkin`
- `Signedness`: `Unsigned`

## Composition

Производится композиция жирных кислот в ТАГи по следующей общей формуле:

- `[abc] = [a13] * [b2] * [c13]`[^4]

### Group

Группирует (опционально) ТАГи:

- `PTC` - группирует по Позиционно-Типовому признаку
- `ECN` - группирует по Эквивалентному Углеродному Числу

`ECN` расчитывается по формуле:

`ECN = CN - 2 * DB`

где `CN` - количество атомов углерода, `DB` - количество двойных связей.

### Sort

Сортирует ТАГи:

- по ключу. В качестве ключа выступает имя группы или индекс жирной кислоты.
- по значению. В качестве ключа при сортировке значение. При эквивалентности
  значений сортировка производится по ключу.

### Order

Задает порядок сортировки:

- `Ascending` - от минимального к максимальному,
- `Descending` - от максимального к минимальному.

## Comparison

Производится сравнение скомпанованных ТАГов.

### Group

Группировка аналогична случаю с композицией, с дополнительной возможной группой:

`CMN` - группирует по Сравнительному Мажорному Числу

`CMN` идентифицирует в каких файлах значение для ТАГа осталось неотфильтрованным
(мажорное), а в каких отфильтрованным (минорное).

## Filtration

Вкладка настроек фильтрации открывается, когда открыта основная вкладка
`Composition` и (или) `Comparison`. В ней задаются настройки фильтрации списка
ТАГов.

Фильтруется по

Если установлен флаг `Mirror`, то расчитывается позиционно-видовой состав.
Другими словами - зеркальные ТАГи объединяются в один, а их значения
суммируются.

То есть формулы расчета композиции преобразуются в частные:

\frac {AAB} {BAA}

- $[AAA] = [A]^3$
- $\left[\begin{array}{ccc}AAB\\ BAA\end{array}\right] \equiv [AAB] + [BAA] = 2 * [A]_{1,3}^2 * [B]_2$[^1]
- $\left[\frac {ABC} {CBA}\right] \equiv [ABC] + [CBA] = 2 * [A] * [B] * [C]$[^1]

- $[A_BAB^A] \equiv [AAB] + [BAA] = 2 * [A]_{1,3}^2 * [B]_2$[^1]

- [aaa] = [a13]^3 [^1]
- [abc] = [abc] + [cba] = 2 * [a13] * [b2] * [c13][^1]
- [aab] = [aab] + [baa] = 2 * [a13] * [a2] * [b13][^1]

остальные варианты расчитываются по общей формуле.

Если установлен флаг `Symmetrical`, то остаются только симметричные ТАГи.
Несимметричные ТАГи фильтруются.

То есть формулы расчета композиции ТАГов преобразуются в частные:

- `[aba] = [a13]^2 * [b2]`[^1]

остальные варианты отсеиваются.

### Settings

## Visualization

Полученные на предыдущих итерациях данные представляются в графическом виде.

## Comparison

123

[^1]: DOI [10.1023/a:1016732708350]
[^pin341.for/56]: `MEC(I) = 4 * MAL(I) - 3 * MBL(I)`: Fortran program [PIN341.for], page 56
[^pin341.for/68]: `MED(I) = 3 * MBL(I) - 2 * MAL(I)`: Fortran program [PIN341.for], page 68
[^4]: DOI [10.1007/s11746-014-2553-8]

[pin341.for]: doc/PIN341.for "Fortran program \"PIN341.for\""
[10.1007/s11746-014-2553-8]: https://doi.org/10.1007/s11746-014-2553-8 "Positional-Species Composition of Triacylglycerols from the Arils of Mature Euonymus Fruits"
[10.1023/a:1016732708350]: https://doi.org/10.1023/a:1016732708350 "Determination of the Positional-Species Composition of Plant Reserve Triacylglycerols by Partial Chemical Deacylation"
[10.1016/s0176-1617(99)80039-x]: https://doi.org/10.1016/s0176-1617(99)80039-x "Developmental Changes in the Triacylglycerol Composition of Sea Buckthorn Fruit Mesocarp"

<!-- [^pin341.for/56]: `MEC(I) = 4 * MAL(I) - 3 * MBL(I)`: Fortran program [PIN341.for], page 56
[^pin341.for/68]: `MED(I) = 3 * MBL(I) - 2 * MAL(I)`: Fortran program [PIN341.for], page 68
[^10.1023/a:1016732708350/706/1]: `[S]₂ = 4[S]₁₂ – 3[S]₁₂₃`: DOI [10.1023/a:1016732708350], page 706, formula 1
[^10.1023/a:1016732708350/706/2]: `[aaa] = [a]₁₃² * [a]₂`: DOI [10.1023/a:1016732708350], page 706, formula 2
[^10.1023/a:1016732708350/706/3]: `[aab] = 2[a]₁₃ * [a]₂`: DOI [10.1023/a:1016732708350], page 706, formula 3
[^10.1023/a:1016732708350/706/4]: `[a]₁₃ = 3[a]₁₂₃ − [a]₂`: DOI [10.1023/a:1016732708350], page 706, formula 4
[^10.1023/a:1016732708350/706/5]: `[a]₁₃ = 3[a]₁₂₃ − [a]₂`: DOI [10.1023/a:1016732708350], page 706, formula 5
[^10.1023/a:1016732708350/706/6]: `[a]₁₃ = (3[a]₁₂₃ − [a]₂) / 2`: DOI [10.1023/a:1016732708350], page 706, formula 6
[^10.1007/s11746-014-2553-8]: DOI [10.1007/s11746-014-2553-8]
[^10.1007/s11746-014-2553-8/2056]: `[a]₁₃ = (3[a]₁₂₃ − [a]₂) / 2`: DOI [10.1007/s11746-014-2553-8], page 2056 -->