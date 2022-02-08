#include "binary.h"
#include <string>
#include <math.h>

namespace binary {

    int convert(const std::string& n){

        int result = 0;

        for(size_t i=0; i < n.length(); i++){

            if(n[i] != '0' && n[i] != '1')
                return 0;

            if (n[i] == '1'){
                result += pow(2, n.length()-i-1);
            }
        }

        return result;

    }

}  // namespace binary
