=====================================================================
===========

Data skall identifieras med hash av innehåll.
Sökvägar är bara en dekoration. Detta gör att cachning fungerar smart.

Data-rör (Putki):

Output realm:
	- Runtime (platform/debug-release)
	- Editor
	
	Input-Desc: (+hash) (+ typs&sökväg) (+ literal content)
	Op-Desc: (hash) 
		InputDesc + PipelineDesc + Realm

	Op-Result: Op-Desc + Input-Desc + Input-View + Op-Result-Content
		

	Op-Cache-Desc: Op-Desc + Input-Desc
		Input-Desc: Synligt input state
			Synligt input state:
				Hash av alla relevanta inputs 
				Eller en Op-Desc

	Op-Result-Content:
			Main-Obj-Result
			Arry of [Input-Desc]

	Output-Obj-Ref: Op-Cache-Desc + Subref
		SubRef:
			0 = objektresultat, samma "typ" som in-datan (ej nödvändigtt)
			1 = additional 'Input-Desc'
				- för att resolva dessa behövs samma
				  operationer men rekursivt

Rot-objekt:

	Kan laddas genom sökväg + pipeline version + realm + [subpath(index?)] => Op-Desc + subpath => Output-Obj-Ref
	Op-Desc säger hur asseten skall produceras
	Op-Desc gör att pipelinen kan bygga den
	Default: Ladda <plattform> + <realm>

Utdataformats
:
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

	load_object(path)
		=> expandera pipeline-version 'default'
		=> expandera plattform 'mac'
		=> konstruera en op-desc
			Blockera på inladdning av resultat (bygg)
			Resultat-objektet resolva:
				- För en länk till op-desc: Bygg
				- För ett index: Ladda in
	
		* Det finns flera koncept för loaders:
			1. Ladda från en färdig databas
			2. Strömma in dynamiskt från en pipeline

			Dessa två implementationer bör kunna existera parallelt och transparent.
			Nummer 1 kommer kunna vara 'snabb' och troligen synkron
			Nummer 2 kommer kunna ta längre tid
			Men det handlar ändå om laddningstider och det måste kunna ske asynkront på något vis.

			Använd ett async-api.
			En första implementation kan vara synkron ändå.


Ideer:
	- En editor kan använda sig av (och bygga) det processade resultatet
          genom att veta hur man frågar efter detta.

	- Runtime behöver en kanal för notifieringar om ändrad input-data
	+ en optimerad bevakning för input-desc








op_desc
	- en typ vars implementation är lite flytande beroende på vart den används
          men kontexten avgör


PutkEnv
	With OpDesc =	


PutkFull
PutkRuntime
PutkStrippedRuntime
PutkEditor
