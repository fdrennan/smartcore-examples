use smartcore::dataset::iris::load_dataset;
// DenseMatrix wrapper around Vec
use smartcore::linalg::naive::dense_matrix::DenseMatrix;
// ndarray wrapper
use ndarray::Array;
// Imports for KNN classifier
use smartcore::neighbors::knn_classifier::*;
use smartcore::math::distance::Distances;
// Imports for Logistic Regression
use smartcore::linear::logistic_regression::LogisticRegression;
// Model performance
use smartcore::metrics::accuracy;

pub fn iris_knn_example(){    
    // Load Iris dataset 
    let iris_data = load_dataset();
    // Turn Iris dataset into NxM matrix
    let x = DenseMatrix::from_array(iris_data.num_samples, iris_data.num_features, &iris_data.data);
    // These are our target class labels
    let y = iris_data.target;

    // Fit KNN classifier to Iris dataset
    let knn = KNNClassifier::fit(
        &x,
        &y,        
        Distances::euclidian(), // We use euclidian distance here. 
        Default::default()
    );
    let y_hat = knn.predict(&x); // Predict class labels

    // Calculate training error
    println!("accuracy: {}", accuracy(&y, &y_hat)); // Prints 0.96
}

pub fn iris_lr_example(){    
    // Load Iris dataset 
    let iris_data = load_dataset();
    // Turn Iris dataset into NxM matrix
    let x = DenseMatrix::from_array(iris_data.num_samples, iris_data.num_features, &iris_data.data);
    // These are our target class labels
    let y = iris_data.target;

    // Fit Logistic Regression to Iris dataset
    let lr = LogisticRegression::fit(&x, &y);
    let y_hat = lr.predict(&x); // Predict class labels

    // Calculate training error
    println!("accuracy: {}", accuracy(&y, &y_hat)); // Prints 0.98
}

pub fn iris_lr_ndarray_example(){    
    // Load Iris dataset 
    let iris_data = load_dataset();
    // Turn Iris dataset into NxM matrix
    let x = Array::from_shape_vec((iris_data.num_samples, iris_data.num_features), iris_data.data).unwrap();

    // These are our target class labels
    let y = Array::from_shape_vec(iris_data.num_samples, iris_data.target).unwrap();

    // Fit Logistic Regression to Iris dataset
    let lr = LogisticRegression::fit(&x, &y);
    let y_hat = lr.predict(&x); // Predict class labels

    // Calculate training error
    println!("accuracy: {}", accuracy(&y, &y_hat)); // Prints 0.98
}