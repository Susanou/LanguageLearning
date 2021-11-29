#include "space_age.h"

namespace space_age {
    const double EARTH = 31557600;
    
    space_age::space_age(long long new_age){

    age = new_age;

    }

    long long space_age::seconds() const{
        
        return age;
    }

    double space_age::on_earth() const{
        return age / EARTH;

    }

    double space_age::on_mercury() const{

        return age / 0.2408467 / EARTH;

    }

    double space_age::on_venus() const{
        return age / 0.61519726 / EARTH;
    }

    double space_age::on_mars() const{

        return age / 1.8808158 / EARTH;

    }

    double space_age::on_jupiter() const{

        return age / 11.862615 / EARTH;

    }

    double space_age::on_saturn() const{

        return age / 29.447498 / EARTH;

    }

    double space_age::on_uranus() const{

        return age / 84.016846 / EARTH;

    }

    double space_age::on_neptune() const{


        return age / 164.79132 / EARTH;  
    }
}  // namespace space_age
