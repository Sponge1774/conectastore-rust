# ConectaStore (Rust)

Sistema de recomendação de produtos baseado em grafos, em Rust — grafo em
lista de adjacência, BFS/DFS e HashMap/HashSet. Projeto da disciplina
**Data Structure Strategy and Implementation** (UniFECAF).

## Estrutura deste repositório
```
.
├── megastore/                          (código-fonte, testes e README técnico)
├── ConectaStore_Relatorio_Tecnico.pdf   (Parte Teórica — relatório completo)
├── prints/                              (evidências de execução, também no Apêndice B do PDF)
│   ├── figura-B1-compilacao-testes.png
│   ├── figura-B2-demo-benchmarks.png
│   └── figura-B3-especificacoes-maquina.png
└── README.md                            (este arquivo)
```

O código, as instruções de compilação/execução, a arquitetura da solução
e os resultados de desempenho estão detalhados em `megastore/README.md`.
Este arquivo é só um mapa do repositório.

## Relatório técnico
[`ConectaStore_Relatorio_Tecnico.pdf`](./ConectaStore_Relatorio_Tecnico.pdf)
— relatório completo (fundamentação teórica, modelagem, análise de
complexidade, desempenho e escalabilidade), incluído aqui por
conveniência. É a mesma entrega feita separadamente como Parte Teórica.

## Prints de execução
A pasta [`prints/`](./prints) traz, como arquivos de imagem individuais,
as mesmas capturas de tela que aparecem no Apêndice B do relatório:
compilação e os 14 testes automatizados passando, a demonstração
funcional com as tabelas de desempenho, e as especificações da máquina
usada nas medições (Samsung Expert 2018, Intel Core i7-8550U, 11 GiB RAM).

## Link do vídeo pitch
Apresentação do projeto rodando no VS Code (até 4 minutos):
https://youtu.be/xT1W0b_cho4
