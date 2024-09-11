abbreviation = abbreviation
common_name = common name
edit = edit
experimental = experimental
fatty_acid_mass = { -fatty_acid_term } { mass }
formula = formula
mass = mass
methyl_ester_mass = methyl ester { mass }
molar_mass = molar { mass }
names = names
names_description = show names for fatty acids
properties = properties
properties_description = show { properties }
resize = resize table columns
selectivity_factor = selectivity factor
species = species
systematic_name = systematic name
theoretical = theoretical

fatty_acid = { -fatty_acid_term }
    .abbreviation = FA
triacylglycerol = triacylglycerol
    .abbreviation = TAG
diacylglycerol = diacylglycerol
    .abbreviation = DAG
monoacylglycerol = monoacylglycerol
    .abbreviation = MAG

configuration = configuration
calculation = calculation
composition = composition

# Central panel

# Left panel
precision = precision
percent = percent

## Calculation
fraction = fraction
as_is = as is
to_mass_fraction = to mass fraction
to_mole_fraction = to mole fraction
sign = sign
signed = signed
    .description = theoretically calculated negative values are as is
unsigned = unsigned
    .description = theoretically calculated negative values are replaced with zeros
from = calculate from
    .description = calculate 1,3-{ diacylglycerol.abbreviation }s from
from_dag = from 1,2/2,3-{ diacylglycerol.abbreviation }s
    .description = calculate 1,3-{ diacylglycerol.abbreviation }s from 1,2/2,3-{ diacylglycerol.abbreviation }s
from_mag = from 2-{ monoacylglycerol.abbreviation }s
    .description = calculate 1,3-{ diacylglycerol.abbreviation }s from 2-{ monoacylglycerol.abbreviation }s

## Composition
adduct = adduct
method = method
gunstone = Gunstone
    .description = calculate by { gunstone }'s theory
vander_wal = Vander Wal
    .description = calculate by { vander_wal }'s theory
group = group
sort = sort
by_key = key
    .description = sort by key
by_value = value
    .description = sort by value
order = order
ascending = ascending
    .description = reverse order (from max to min)
descending = descending
    .description = direct order (from min to max)

key = key
value = value

-fatty_acid_term = fatty acid
