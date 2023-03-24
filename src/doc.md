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

O método `clear()` é uma implementação para limpar o tabuleiro de xadrez e redefinir várias variáveis de estado do jogo. Aqui está uma explicação de cada linha no método:

1. `this._board = new Array<Piece>(128)`: Inicializa a propriedade `_board` em um array vazio de 128 elementos, representando o tabuleiro de xadrez no formato 0x88.
2. `this._kings = { w: VAZIO, b: VAZIO }`: Redefine a propriedade `_kings`, que armazena as casas onde os reis estão localizados, para valores vazios para branco e preto.
3. `this._turn = WHITE`: Define a propriedade `_turn` como WHITE, indicando que é a vez do jogador branco jogar.
4. `this._castling = { w: 0, b: 0 }`: Redefine a propriedade `_castling`, que armazena os direitos de roque para branco e preto, para 0 (sem direitos de roque).
5. `this._epSquare = EMPTY`: Redefine a propriedade `_epSquare`, que armazena o quadrado de destino en passant, para um valor vazio.
6. `this._halfMoves = 0`: Redefine a propriedade `_halfMoves`, que conta o número de meios movimentos desde o último movimento ou captura do peão (usado para a regra de 50 movimentos).
7. `this._moveNumber = 1`: redefine a propriedade `_moveNumber`, que conta o número de jogadas completas no jogo, para 1.
8. `this._history = []`: Redefine a propriedade `_history`, que armazena o histórico de jogadas no jogo, para um array vazio.
9. `this._updateSetup()`: Chama o método `_updateSetup` para atualizar o estado interno do jogo com base na nova configuração do tabuleiro.

Essa implementação assume que a classe Chess possui as propriedades e constantes correspondentes definidas. Você precisaria adicionar essas propriedades à classe Chess e definir as constantes (por exemplo, WHITE, EMPTY) para que o método funcione corretamente.

perft(profundidade: número): Executa um teste de desempenho contando todos os movimentos possíveis até a profundidade dada.