abbreviation = aббревиатура
common_name = общепринятое название
names = названия
properties = свойства
systematic_name = систематическое название

fatty_acid_mass = масса { -fatty_acid(genus: "genitive") } 
methyl_ester_mass = масса метилового эфира

-fatty_acid = { $genus ->
   *[nominative] жирная кислота
    [genitive] жирной кислоты
}