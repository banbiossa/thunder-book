#ifndef SRC_CH06_MCTS_NODE_H_
#define SRC_CH06_MCTS_NODE_H_

#include "maze_state.h"
#include "mcts_base.h"

// forward declaration for the class
class OddNode;

class EvenNode : public BaseNode<OddNode>
{
public:
    EvenNode(const State &state) : BaseNode(state){};
    void expand() override;
    double explore() override;
    int best_action(int player_id) const;
};

class OddNode : public BaseNode<EvenNode>
{
private:
    int action0; // action of previous node

public:
    OddNode(const State &state, int action0) : BaseNode(state),
                                               action0(action0){};
    void expand() override;
    double explore() override;
};

int mcts_action(State state, int player_id, int playout_number);

#endif
