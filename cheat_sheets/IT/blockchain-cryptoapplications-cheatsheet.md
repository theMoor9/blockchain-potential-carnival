# **Blockchain CheatSheet - Cryptoapplicazioni**

<div style="font-size: 70%"><b>&#x1F553; Tempo di Lettura: 8 m</b></div>

---

##### **Indice**

###### [§ Denaro e Cripto](#-Denaro-e-Cripto-1)
- [Denaro](#Denaro)
- [Acquisire Criptovaluta](#Acquisire-Criptovaluta)
- [Scambi di Cripto](#Scambi-di-Cripto)
###### [§ Stablecoins](#-Stablecoins-1)
- [Volatilità delle Criptovalute](#Volatilit%C3%A0-delle-Criptovalute)
- [Collateralizzati in FIAT](#Collateralizzati-in-FIAT)
- [Collateralizzati in Asset Reali](#Collateralizzati-in-Asset-Reali)
- [Collateralizzati in Cripto](#Collateralizzati-in-Cripto)
- [Non Collateralizzati](#Non-Collateralizzati)
###### [§ Offerte Iniziali di Monete (ICO)](#-Offerte-Iniziali-di-Monete-ICO-1)
- [Investitori](#Investitori)
- [Criteri di Valore](#Criteri-di-Valore)
- [Trend ICO](#Trend-ICO)
###### [§ Cripto Fraudolente](#-Cripto-Fraudolente-1)
- [NIENTE SCHEMI PONZI](#NIENTE-SCHEMI-PONZI)
- [NIENTI RENDIMENTI SURREALI](#NIENTI-RENDIMENTI-SURREALI)
- [NIENTE INCONTRI EMOTIVI/OFFERTI SU HYPE](#NIENTE-INCONTRI-EMOTIVI-OFFERTI-SU-HYPE)
- [NIENTI STRANGER SUI SOLDI](#NIENTI-STRANGER-SUI-SOLDI)
- [ALLERTA ROSSA](#ALLERTA-ROSSA)
###### [§ Framework di Valutazione](#-Framework-di-Valutazione-1)
- [App](#App)
- [Foglio](#Foglio)

<hr style="page-break-before: always; ">

## **§ Denaro e Cripto**

### Denaro

**Scopi del Denaro**:

- Unità di misura per confrontare il valore di beni e servizi.
- Mezzo di scambio come alternativa più efficiente al baratto.
- Riserva di valore da conservare nel tempo, parzialmente, invece di un completo decadimento come il cibo.
- Trasferimento facile del valore.

> Il trasferimento facile del valore è l'idea fondamentale dietro Bitcoin.

### Acquisire Criptovaluta

- **Mining**: Partecipare alla rete per convalidare le transazioni e minare nuovi blocchi.
- **Regali o Pagamenti**: Ricevere criptovaluta come forma di pagamento o come regalo.
- **Scambi e Portafogli Moderni**: Acquistare criptovalute utilizzando piattaforme simili ai broker di azioni.
- [**Offerte Iniziali di Monete (ICO)**](#-Offerte-Iniziali-di-Monete-ICO-1): Investire in un nuovo progetto di criptovaluta partecipando alla sua offerta iniziale.

### Scambi di Cripto

A differenza degli scambi tradizionali di azioni, gli scambi di criptovalute spesso aderiscono a rigide politiche di Conosci il Tuo Cliente (KYC) a causa dei requisiti normativi associati alla blockchain e alle criptovalute.

È importante notare che questi scambi non sono privi di rischi. Le sfide includono [truffe](#-Cripto-Fraudolente-1), hack e pratiche commerciali scadenti. È necessaria una valutazione attenta quando si sceglie uno scambio per garantire sicurezza e affidabilità.

---

## **§ Stablecoins**

### Volatilità delle Criptovalute

Le criptovalute sono rinomate per essere uno dei tipi di asset più volatili in circolazione. Infatti, durante la crisi finanziaria del 2008, i mercati hanno visto cali di valore fino al 9% in una sola settimana; un evento incredibile. Ma, che ci crediate o no, ciò è nulla rispetto al flusso normale di liquidità nei mercati delle criptovalute. Se sei qui a leggere questo, potresti già sapere delle sue oscillazioni incredibili.

In ordine di complessità, ecco i quattro tipi di Stablecoins:

1. [Collateralizzati in FIAT](#Collateralizzati-in-FIAT)
2. [Collateralizzati in Asset Reali](#Collateralizzati-in-Asset-Reali)
3. [Collateralizzati in Cripto](#Collateralizzati-in-Cripto)
4. [Non Collateralizzati](#Non-Collateralizzati)

### Collateralizzati in FIAT

##### L'idea

Gli Stablecoins Collateralizzati in FIAT sono progettati per fornire a tutti accesso a trasferimenti ottimizzati in qualsiasi momento, inclusi transazioni a basso costo in orari insoliti, come le 02:00 di una domenica mattina. Questo approccio affronta in parte il problema dell'alta volatilità, come quella vista con gli scambi di Bitcoin.

Inoltre, se l'emittente degli stablecoins che hai scambiato per le monete legate al FIAT incontra difficoltà finanziarie, dovresti essere in grado di recuperare il valore in FIAT.

### Collateralizzati in Asset Reali

Un esempio principale è Digix Gold (DGX) che 1DGX equivale a 1 grammo d'oro.

- Utilizza il protocollo Ethereum Blockchain ERC-20.
- Mantiene il proprio caveau a Singapore.
- Riscattabile in lotti di 100 o 1000 grammi.
- Le commissioni includono i costi delle transazioni e del deposito dell'oro fisico.

##### Altro

- Swiss Real Coin (SRC): Supportato da un portafoglio immobiliare svizzero.
- D1 Coin ERC-20: Collateralizzato da diamanti.

### Collateralizzati in Cripto

Questo tipo di Stablecoin funziona in modo simile a quelli ancorati a un valore FIAT ma aggiunge una curiosa variazione basata sulla criptovaluta scelta.

**Come funziona** _Supponiamo che 1 Stablecoin (STB) = 1 SOL_

1. **Acquisto Iniziale**: Quando acquisti STB, la tua transazione coprirà più dell'importo equivalente in SOL per includere un margine per le fluttuazioni di mercato.
    
    yaml
    
    Copia codice
    
     `Necessario: 50 STB  Acquista: 75 SOL  In Portafoglio:  	- Totale: 50 STB/SOL in sospeso + 25 SOL  	- Ripartizione: 50 STB e 25 SOL (il surplus SOL funge da buffer)`
    
2. **Collateralizzazione**: A questo punto, il valore di SOL diventa collateralizzato e se il valore della cripto scende abbastanza, il tuo SOL corrispondente agli STB verrà liquidato dai _custodi_ delegati degli stablecoin del contratto per acquistare STB e aumentare la stabilità del sistema.
    
    yaml
    
    Copia codice
    
     `Scenario di Liquidazione: 50 SOL liquidati per mantenere il peg degli STB  Importo Effettivo in Portafoglio: 50 STB e 25 SOL`
    
3. **Aumento del Valore e Vendita**: Se il valore di SOL aumenta e decidi di vendere, puoi convertire i tuoi STB collateralizzati in SOL più preziosi o anche in FIAT.
    
    yaml
    
    Copia codice
    
     `Aumento: SOL +50%  Vendi: 50 STB di nuovo in FIAT o SOL, beneficiando del valore aumentato  Saldo Risultante: Guadagno dal valore aumentato di SOL, più SOL rimanenti`
    

> Se il sistema affronta una minaccia significativa o instabilità, potrebbe attivarsi una Liquidazione Globale, restituendo la cripto collateralizzata ai suoi detentori originali.

### Non Collateralizzati

Questo tipo di Stablecoin opera attraverso meccanismi interni per garantire che il suo prezzo rimanga stabile.

**Come funziona**

- **Meccanismi di Regolazione**: L'organo regolatore della stablecoin potrebbe intervenire nel mercato creando o riacquistando monete per regolare l'offerta e stabilizzare il valore.
- **Conversione in Token di Obbligazioni**: Le stablecoin acquistate possono essere convertite in token di obbligazioni, che rimuovono temporaneamente la liquidità dal mercato e stabilizzano il valore della moneta interrompendo la generazione di rendimento.
- **Conversione in Token Azionari**: In alternativa, le stablecoin acquistate possono essere convertite in token azionari, distribuendo il valore immagazzinato tra i detentori che preferiscono mantenere la proprietà durante le fluttuazioni di mercato.

##### Meccanismi Aggiuntivi

Alcuni token stabili sfruttano i framework di programmazione delle blockchain esistenti per creare una versione personalizzata di criptovaluta modificando il codice per alterare le dinamiche economiche.

**Come funziona**

- **Regolazioni dell'Offerta**: Se il valore aumenta, viene introdotta un'offerta aggiuntiva per moderare il prezzo.
- **Regolazioni del Mining**: Se il valore diminuisce, le ricompense del mining possono essere ridotte, potenzialmente a zero, per scoraggiare l'offerta eccessiva.
- **Utilizzo delle Commissioni**: Le commissioni possono essere raccolte per tamponare contro le diminuzioni di valore e possono essere distrutte in scenari in cui il valore aumenta eccessivamente, aiutando a mantenere la stabilità.

> Le stablecoin non collateralizzate spesso portano un valore 'fittizio' poiché non sono supportate da beni tangibili o garanzie tradizionali, basandosi invece su interventi algoritmici di mercato per mantenere il loro peg.

---

## **§ Offerte Iniziali di Monete (ICO)**

### Investitori

I movimenti delle Offerte Iniziali di Monete possono essere causati da diversi fattori

**Manipolazione da Parte dei Grandi Investitori**:

_"...l'aumento del prezzo ha generato entusiasmo aggiuntivo per l'acquisto, attirando acquirenti che vedono l'aumento come una conferma di una tesi di investimento. Man mano che gli investitori 'sulla moda' si uniscono a qualsiasi festa, creano la loro verità—per un po'."_

Buffett non parlava di bitcoin – stava parlando dell'oro nel 2012.

> I grandi investitori potrebbero molto bene manipolare il mercato con questa verità.

**Investitori alla Moda**:

Quelli che vedono l'aumento del prezzo e acquistano ad alti livelli contribuendo in certa misura alla continuazione dell'aumento dei prezzi e poi inevitabilmente vendono a bassi livelli.

> Voglio dire, siamo qui per non essere loro.

### Criteri di Valore

1. **Prima Gen/Gold 2.0**: Come meccanismo di transazione.
2. **Token Computazionali Distribuiti**: Come tecnologie potenti.
3. **Token di Utilità**: Come estensione degli asset tecnologici.
4. **Token di Sicurezza**: Come tokenizzazione di valori esistenti come azioni, obbligazioni o altri asset.
5. **Token Fungibili**: Con valore mutevole basato sul caso d'uso.
6. **Token Non Fungibili**: Beh...
7. **Stablecoin**: Più facili da valutare per natura.

### Trend ICO

Possiamo certamente capire che le ICO non si fermeranno finché ci saranno nuove tecnologie innovative in cui investire.

---

## **§ Cripto Fraudolente**

Voglio infondere in questa sezione un tocco personale e informale perché è profondamente personale. Ho visto amici cadere vittime di questi schemi, che hanno preso di mira me o addirittura la mia famiglia più volte. Fortunatamente, la mia comprensione delle tattiche di manipolazione e del controllo emotivo ci ha aiutati a proteggerci. Non sopporto davvero coloro che sfruttano le debolezze emotive per esercitare potere.

### **NIENTE SCHEMI PONZI**

**Nessun lavoro dovrebbe rimanere non pagato** se sta generando denaro. Non aspettarti ritorni su lavori che hai fatto gratuitamente.

### **NIENTI RENDIMENTI SURREALI**

**Fai i conti** prima di investire tempo e denaro. È intrinsecamente impossibile avere prezzi premium o profitti garantiti nei mercati di scambio.

### **NIENTE INCONTRI EMOTIVI/OFFERTI SU HYPE**

**Le emozioni possono offuscare la realtà.** Anche se le emozioni sono un aspetto fondamentale dell'esperienza umana, possono offuscare il giudizio.

_Come su una scala, l'emozione può portare alla rovina della percezione della realtà; più ti dirigi verso l'emozione, più è probabile che la tua visione diventi sfocata._

Questi incontri sono spesso teatri meticolosamente progettati per manipolarti. Caratteristiche comuni includono:

- **Incontri Tardivi**: Le tue difese sono più basse quando sei stanco.
- **Parlare Rapidamente**: Consegna veloce di grandi numeri e calcoli complessi lascia poco spazio al pensiero critico.
- **Sfruttamento dell'Influenza della Folla**: Spesso possono essere presenti attori per dare un'atmosfera più credibile. Questi "attori" o individui completamente manipolati cercheranno di indirizzarti verso i loro ideali.
- **Tentativo di Coinvolgimento Emotivo**: Parlano di debolezze sociali comuni usando immagini impattanti per penetrare efficacemente le tue difese, poiché queste sono assimilate più immediatamente da una mente stanca.

### **NIENTE STRANGER SUI SOLDI**

**Cerca nomi e persone, reputazioni e identità** spesso usano immagini e nomi falsi per simulare credibilità creando personaggi fittizi che non potresti mai mettere in discussione o consultare.

### **ALLERTA ROSSA**

_**La presenza di una qualsiasi di queste caratteristiche dovrebbe essere considerata un'allerta rossa. Rimani vigile e metti in discussione tutto.**_

---

## **§ Framework di Valutazione**

_**"Dobbiamo esaminare attentamente le Offerte Iniziali di Monete, essere preparati, evitare errori, per investire in modo incisivo"**_

Segue una piccola applicazione che ho costruito nel linguaggio di programmazione Rust (Tecnologia utilizzata da Solana) che ti aiuta a valutare meglio le ICO con un foglio di spiegazione completo.

_(Questo è ispirato da una checklist esaminata da capitalisti di rischio e migliorata dal Dr. Harvey R. Campbell)_

### [App](../../code/evframework/main.exe)

clicca sul titolo

### [Foglio](./evaluation_sheet.md)

clicca sul titolo

_(Questo è un checklist semplificata ispirata a una versione esaminata da capitalisti di rischio e migliorata da Harvey Campbell.)_

---

###### Seguito Suggerito

[Blockchain Cheat Sheet - Futuro del Fintech](./blockchain-fintech-cheatsheet.md)

---

**Autore**: Kenneth Boldrini