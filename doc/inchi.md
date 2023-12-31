# IUPAC Standard InChI

Каждый InChI начинается со

- `InChI=`;
- номер версии, в настоящее время `1`;
- `S` для стандартных InChI, которые представляют собой полностью
стандартизованный вариант InChI, сохраняющий тот же уровень внимания к деталям
структуры и те же соглашения для восприятие рисунка.

Оставшаяся информация структурирована как последовательность уровней и
подуровней, каждый из которых предоставляет один определенный тип информации.

Слои и подслои разделяются разделителем `/` и начинаются с характерной буквы
префикса (за исключением подслоя химической формулы основного слоя). Шесть слоев
с важными подслоями:

1. Основной слой

  - Химическая формула (без префикса). Это единственный подуровень, который
должен встречаться в каждом InChI.
  - Подключения Atom (префикс: «c»). Атомы в химической формуле (кроме атомов
водорода) пронумерованы последовательно; этот подслой описывает, какие атомы
связаны связями с какими другими.
  - атомы водорода (префикс: «h»). Описывает, сколько атомов водорода связано с
каждым из других атомов.

2. Зарядный слой
подслой протонов (префикс: «p» для «протонов»)
подслой заряда ( префикс: "q")
стереохимический слой
двойные связи и кумулены (префикс: «b»)
тетраэдрическая стереохимия атомов и алленов ( префиксы: «t», «m»)
тип стереохимической информации (префикс: «s»)
Изотопный слой (префиксы: «i», «h», а также « b »,« t »,« m »,« s »для изотопной стереохимии)
Слой фиксированного H (префикс:« f »); содержит некоторые или все вышеперечисленные типы слоев, за исключением соединений атомов; может заканчиваться подслоем «o»; никогда не входил в стандартный InChI
повторно подключаемый слой (префикс: "r"); содержит весь InChI структуры с пересоединенными атомами металла; никогда не включается в стандартный InChI

[wikibrief.org](https://ru.wikibrief.org/wiki/International_Chemical_Identifier)
