=====================================================================
===========

Data skall identifieras med hash av innehåll.
Sökvägar är bara en dekoration. Detta gör att cachning fungerar smart.

Data-rör (Putki):

Output realm:
	- Runtime (platform/debug-release)
	- Editor
	
	Input: hash (+ sökväg)
	Output: hash (+ sökväg)
	Op-Desc: (hash) 
		input data + pipeline version + pipeline realm 
	Op-Cache-Desc: 
		Op-Desc => Result + Input-Desc 
		Input-Desc: Synligt input state
			Synligt input state:
				Hash av alla relevanta inputs 
				Eller en Op-Desc

Rot-objekt:

	Kan laddas genom sökväg + pipeline version + realm => Op-Desc
	Op-Desc säger hur asseten skall produceras
	Op-Desc gör att pipelinen kan bygga den
	Default: Ladda <plattform> + <realm>

Utdataformat:
	- Databas med op-desc 
	- En header med vilken version av alla pipelines som 'default' är

Runtime:
	- Alla versioner är 'default' vilket innebär att det är senaste
	  versionen.
	- Hur betrakta 'gammal' databas? 

Live-update:
	- Kom ihåg namnet på resurser som laddat (sökväg) 
	- Övervaka dess iput-desc och ifall någonting ändrats så efterfråga
	  en ny version
	- Byt ut versionen och uppdatera

Komponenter:

	- Input-databas:
		Redigera data - event 
		Läs data - operation
	- Cache-databas:
		Skriv utdata - pipeline



Runtime:

	Läs ur cache-databasen
		Input-Desc => Resultat

	Cache-databas:
		Pekare: 
			=> index till Hash av Op-Desc (debug)
			=> index till objekt i databasen (optimerad)



Ideer:
	- En editor kan använda sig av (och bygga) det processade resultatet
          genom att veta hur man frågar efter detta.

	- Runtime behöver en kanal för notifieringar om ändrad input-data
	+ en optimerad bevakning för input-desc