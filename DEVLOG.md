Chip8 in realtà è un linguaggio di programmazione interpretato, per questo motivo in teoria non si costruisce un emulatore, ma un interprete.
Infatti non c'è Hardware da emulare, come nel caso degli emulatori, c'è solo una specifica per il Chip-8. Non esiste un chip.
Quello che si sta realizzando è un interprete di bytecode che gira dentro macchina virtuale. Ma dato che questo interprete si deve occupare di gestire concetti fisici (RAM, timer, display), si comporta come un emulatore.

I linguaggi interpretati, come Python e Javascript, funzionano attraverso un programma che esegue ogni comando riga per riga, un interprete, quello che questo progetto vuole realizzare. Sono quindi anche più lenti dei linguaggi di programmazione compilati.
I linguaggi compilati, come C e RUST, vengono convertiti direttamente in codice macchina, quindi sono più veloci ed efficienti. Permettono allo sviluppatore un controllo maggiore su memoria e CPU.

Un interprete (e anche un emulatore, che in questo caso sono la stessa cosa) è una macchina a stati.
Cos'è una macchina a stati?
In pratica significa che il mio programma si basa su due cose:
- lo stato attuale (com'è la memoria in un esatto momento)
- Un input (istruzione che stiamo leggendo)

Quindi la prima cosa da fare per costruire questo "emulatore" è definire lo stato della macchina, quindi come è fatta la struttura della sua CPU.

Le specifiche del Chip8 sono le seguenti:
Memoria: Un array di byte. Sono 4096
Registri (V0 to Vf): Un array di 16 elementi. Che tipo di dato serve per 8 bit?
Registro Indice (I): Un registro singolo. Serve per gli indirizzi di memoria (16 bit, ma ne usa solo 12 perchè deve arrivare a 4096, memoria massima, che è 2 alla 12)
Program Counter (PC): Un registro singolo per sapere quale istruzione stiamo eseguendo
Stack: Un array per salvare gli indirizzi di ritorno dalle funzioni (il Chip-8 ne supporta solitamente 16)
Stack Pointer (SP): Un numero per sapere a che punto dello stack siamo
Timers: Due timer (Delay e Sound) che scalano a 60Hz