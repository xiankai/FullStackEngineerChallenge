import React from "react";
import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
} from "@material-ui/core";
import { RouteComponentProps } from "react-router";
import AddReviewForm from "./AddReviewForm";

interface Review {
  id: number;
  review: string;
  score: number;
  created: string;
  updated: string;
}

export default function Reviews(
  props: RouteComponentProps<{
    employee_id: string;
  }>
) {
  const [reviews, setReviews] = React.useState<Review[]>([]);
  const employee_id = +props.match.params.employee_id;

  const fetchEmployees = () => {
    fetch("/get_reviews", {
      method: "POST",
      body: JSON.stringify({ employee_id }),
    })
      .then((res) => res.json())
      .then((json) => setReviews(json));
  };

  // Initial load
  React.useEffect(fetchEmployees, []);

  return (
    <>
      <TableContainer component={Paper}>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>Review ID</TableCell>
              <TableCell align="right">Review</TableCell>
              <TableCell align="right">Score</TableCell>
              <TableCell align="right">Date</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {reviews.map((review) => (
              <TableRow key={review.id}>
                <TableCell component="th" scope="row">
                  {review.id}
                </TableCell>
                <TableCell component="th" scope="row">
                  {review.review}
                </TableCell>
                <TableCell align="right">{review.score}</TableCell>
                <TableCell align="right">{review.created}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
      <AddReviewForm employee_id={employee_id} callback={fetchEmployees} />
    </>
  );
}
