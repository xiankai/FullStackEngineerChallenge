import React from "react";
import { Input, Button } from "@material-ui/core";

interface Props {
  employee_id: number;
  callback: () => void;
}

export default function AddReviewForm(props: Props) {
  let [review, setReview] = React.useState("");
  let [score, setScore] = React.useState(5);

  const handleSubmit = () => {
    fetch("/add_review", {
      method: "POST",
      body: JSON.stringify({ employee_id: +props.employee_id, score, review }),
    })
      .then(props.callback)
      .catch((err) => {
        console.error(err);
        alert("error");
      });
  };

  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        handleSubmit();
      }}
    >
      <Input
        value={review}
        placeholder="Review"
        onChange={(e) => setReview(e.target.value)}
        required
      />
      <Input
        value={score}
        type="number"
        placeholder="Score"
        onChange={(e) => setScore(+e.target.value)}
        required
      />
      <Button variant="contained" type="submit">
        Add Review
      </Button>
    </form>
  );
}
