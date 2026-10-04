#!/usr/bin/env bash
# Script de testes do ConectaStore — gera, em ordem, tudo que precisa
# ser capturado em print para o Apêndice B do relatório.
#
# Como usar (Pop!_OS / qualquer Linux com Rust instalado):
#   1. Extraia o ConectaStore_Projeto_Rust.zip
#   2. Copie este arquivo para dentro da pasta "megastore" (mesmo nível do Cargo.toml)
#   3. No terminal, dentro da pasta megastore:
#        chmod +x rodar_testes.sh
#        ./rodar_testes.sh
#
# O script pausa entre cada etapa para você tirar o print com calma.
# Toda a saída também é salva em resultados_conectastore.txt, caso
# prefira me colar o texto em vez de (ou além de) enviar prints.

set -e

if [ ! -f "Cargo.toml" ]; then
  echo "ERRO: rode este script de dentro da pasta 'megastore' (onde está o Cargo.toml)."
  exit 1
fi

echo "===================================================================="
echo " ConectaStore — script de testes para o relatório"
echo "===================================================================="
echo

echo ">>> Verificando o Rust instalado:"
cargo --version
rustc --version
echo

read -p "Pressione ENTER para compilar em modo release... " _
echo
cargo build --release
echo
echo "###  PRINT 1: capture a saída acima (deve terminar em 'Finished release [optimized]')."
echo
read -p "Pressione ENTER para rodar os testes automatizados... " _
echo
cargo test --release
echo
echo "###  PRINT 2: capture a linha 'test result: ok. 14 passed; 0 failed'."
echo
read -p "Pressione ENTER para rodar a demonstração e os benchmarks (saída mais longa)... " _
echo

# Salva a saída completa em arquivo e também mostra na tela.
cargo run --release 2>&1 | tee resultados_conectastore.txt

echo
echo "===================================================================="
echo "Rolando o terminal para cima, capture três prints separados da saída acima:"
echo "###  PRINT 3: catálogo cadastrado + consulta direta + recomendações do Cliente Eduardo"
echo "###  PRINT 4: tabela 'Benchmark: tempo de construção (...) e recomendação por volume' (5 linhas)"
echo "###  PRINT 5: tabela 'Comparação lista × matriz de adjacência (espaço ocupado)' (4 linhas)"
echo "===================================================================="
echo
echo "A saída completa também foi salva em: resultados_conectastore.txt"
echo "Se preferir, me envie o conteúdo desse arquivo em vez dos prints 4 e 5 —"
echo "eu extraio os números de lá para atualizar as Tabelas 1 e 2 do relatório."
