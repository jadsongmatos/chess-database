# Chess

A classe Chess possui vários métodos para lidar com vários aspectos de um jogo de xadrez, como gerenciar o tabuleiro, mover peças, verificar vários estados do jogo (por exemplo, xeque-mate, impasse) e manter um histórico de jogadas.

Aqui está uma breve explicação de cada método na classe Chess:

1. `clear()`: Limpa o tabuleiro de todas as peças.
2. `reset()`: Redefina o quadro para a posição inicial.
3. `get(square: Square)`: Devolve a peça num determinado quadrado.
4. `remove(square: Square)`: Remove uma peça de um determinado quadrado.
5. `isAttacked(square: Square, attackBy: Color)`: Verifica se um quadrado é atacado por uma cor específica.
6. `isCheck()`: Verifica se o rei do jogador atual está em xeque.
7. `isCheckmate()`: Verifica se o rei do jogador atual está em xeque-mate.
8. `isStalemate()`: Verifica se o rei do jogador atual está em impasse.
9. `isInsufficientMaterial()`: Verifica se não há material suficiente na placa para entregar o xeque-mate.
10. `isThreefoldRepetition()`: Verifica se a posição atual ocorreu três vezes no jogo.
11. `isDraw()`: Verifica se o jogo é um empate devido a várias condições de sorteio.
12. `isGameOver()`: Verifica se o jogo acabou devido a xeque-mate, impasse ou empate.
13. `move()`: Faz um movimento no tabuleiro.
14. `undo()`: Desfaz o último movimento feito no tabuleiro.
15. `turn()`: Devolve o turno do jogador atual.
16. `board()`: Devolve o estado atual da placa.
17. `history()`: Retorna o histórico de movimentos feitos no jogo.


perft(profundidade: número): Executa um teste de desempenho contando todos os movimentos possíveis até a profundidade dada.