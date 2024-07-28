//SPDX-License-Identifier: GPL3.0
pragma solidity 0.8.26;

contract Oracle {
  // this contract should communicate with an external server
  // sending a students public key and receiving an encrypted random question/answer pair
    string[] public questions = [ "The answer to this question is 2"
                                , "The answer to this question is 34"
                                ];
    uint8[] public answers = [ 2
                             , 34
                             ];  
}

// each student has a separate contract deployed for them 
// tracking their progress and grade
contract TutorContract {
  type fxp is int64;

  Oracle TutorOracle; 

  string[] questions;
  uint8[] answers;
  fxp[] weights;
  fxp[] points;
  uint8 progress;
  fxp grade;

  event Answered(bool correct);

  constructor(address OracleAddress) payable {
      // here we want to decrypt the encrypted questions and answers sent from the oracle
      TutorOracle = Oracle(OracleAddress);
      questions = [TutorOracle.questions(0), TutorOracle.questions(1)];
      answers = [TutorOracle.answers(0), TutorOracle.answers(1)];

      weights = [fxp.wrap(2000000), fxp.wrap(1500000), fxp.wrap(900000), fxp.wrap(400000), fxp.wrap(200000)];
      points = [fxp.wrap(1000000), fxp.wrap(1000000), fxp.wrap(1000000), fxp.wrap(1000000), fxp.wrap(1000000)];
      progress = 0;
      grade = fxp.wrap(0);
  }

  function fxp_mul(fxp x, fxp y) private pure returns (fxp) {
      return fxp.wrap(fxp.unwrap(x) * fxp.unwrap(y) / 1000000);
  }

  function dot_product(fxp[] memory xs, fxp[] memory ys) private pure returns (fxp) {
      assert(xs.length == ys.length);
      int64 product = 0;
      for (uint8 i = 0; i < xs.length; i++) {
	      product += fxp.unwrap(fxp_mul(xs[i], ys[i]));
      }
      return fxp.wrap(product);
  }

  function push_front(fxp[] memory xs, fxp x) private pure returns (fxp[] memory) {
    fxp[] memory ys = new fxp[](xs.length + 1);

    for (uint256 i = 0; i < xs.length; i++) {
        ys[i+1] = xs[i]; 
    }
    ys[0] = x;

    return ys;
  }

  function calculate_grade() public view returns (fxp) {
      return dot_product(weights, points);
  }

  function get_current_question() external  view returns (string memory) {
      // this data should be specific to the student asking the question
      return questions[progress];
  }

   function get_progress() external  view returns (uint8) {
      return progress;
  }
  // returns correct answer
  function answer(uint8 a, address payable to) external payable returns (uint8) {
      require(msg.value == 0 wei);

      if (a != answers[progress]) {
	      points = push_front(points, fxp.wrap(-2000000));
	      points.pop();
	      emit Answered(false);

        return answers[progress];
      }

      // else if it was correct
      progress++;
      points = push_front(points, fxp.wrap(2000000));
      points.pop();

      if (progress >= questions.length) {
        progress = 0;
      }

      grade = calculate_grade();

      if (fxp.unwrap(grade) > 9750000) {
        (bool sent, ) = to.call{value: 500 wei}("");
        require(sent, "Failed to send Ether");
      } 
      return answers[progress];
    }

   
}

