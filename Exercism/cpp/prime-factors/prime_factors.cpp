#include "prime_factors.h"

#include <vector>


namespace prime_factors {

    std::vector<int> of(int n){
        
        //base case since any number below 2 has no prime factors
        if (n < 2){
            return {};
        }

        std::vector<int> factors;

        int i=2;

        while (i<=n){

            if (!(n%i)){
                factors.push_back(i);
                n /= i;
            }else{
                i++;
            }

        }

        return factors;


    }

}  // namespace prime_factors
